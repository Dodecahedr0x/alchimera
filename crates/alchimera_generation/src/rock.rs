//! Pure deterministic irregular rock assembly data.

use alchimera_core::seed::WorldSeed;

/// Configuration for procedural rock source generation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockConfig {
    pub radial_segments: usize,
    pub vertical_layers: usize,
    pub base_radius: f32,
    pub radius_variation: f32,
    pub height: f32,
    pub harvest_point_count: usize,
}

impl Default for RockConfig {
    fn default() -> Self {
        Self {
            radial_segments: 10,
            vertical_layers: 4,
            base_radius: 1.1,
            radius_variation: 0.35,
            height: 1.4,
            harvest_point_count: 4,
        }
    }
}

/// Axis-aligned bounds for generated rock source geometry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl RockBounds {
    #[must_use]
    pub fn width(self) -> f32 {
        self.max[0] - self.min[0]
    }

    #[must_use]
    pub fn height(self) -> f32 {
        self.max[1] - self.min[1]
    }

    #[must_use]
    pub fn depth(self) -> f32 {
        self.max[2] - self.min[2]
    }

    #[must_use]
    pub fn contains(self, position: [f32; 3]) -> bool {
        position[0] >= self.min[0]
            && position[0] <= self.max[0]
            && position[1] >= self.min[1]
            && position[1] <= self.max[1]
            && position[2] >= self.min[2]
            && position[2] <= self.max[2]
    }
}

/// A point on the rock that gameplay can use for mining hits or loot placement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockHarvestPoint {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// Pure irregular rock mesh-source data.
#[derive(Debug, Clone, PartialEq)]
pub struct RockAssembly {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub bounds: RockBounds,
    pub harvest_points: Vec<RockHarvestPoint>,
}

impl RockAssembly {
    /// Returns a compact deterministic summary for tests, diagnostics, and cache keys.
    #[must_use]
    pub fn summary(&self) -> RockSummary {
        RockSummary {
            vertices: self.vertices.len(),
            triangles: self.indices.len() / 3,
            harvest_points: self.harvest_points.len(),
            width_millimeters: quantize(self.bounds.width()),
            height_millimeters: quantize(self.bounds.height()),
            depth_millimeters: quantize(self.bounds.depth()),
        }
    }
}

/// Compact deterministic summary of generated rock data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RockSummary {
    pub vertices: usize,
    pub triangles: usize,
    pub harvest_points: usize,
    pub width_millimeters: i32,
    pub height_millimeters: i32,
    pub depth_millimeters: i32,
}

/// Generates deterministic irregular rock mesh-source data from a seed and configuration.
#[must_use]
pub fn generate_rock(seed: WorldSeed, config: RockConfig) -> RockAssembly {
    let radial_segments = config.radial_segments.max(3);
    let vertical_layers = config.vertical_layers.max(2);
    let mut vertices = Vec::with_capacity(radial_segments * vertical_layers + 2);

    vertices.push([0.0, config.height, 0.0]);
    for layer in 0..vertical_layers {
        let layer_fraction = layer as f32 / (vertical_layers - 1) as f32;
        let y = config.height * (1.0 - layer_fraction) * 0.86;
        let profile = (std::f32::consts::PI * layer_fraction).sin().max(0.18);
        for segment in 0..radial_segments {
            let angle = segment as f32 / radial_segments as f32 * std::f32::consts::TAU;
            let jitter = signed_unit(
                seed,
                "rock.radius",
                (layer * radial_segments + segment) as u64,
            );
            let radius = (config.base_radius + jitter * config.radius_variation) * profile;
            vertices.push([angle.cos() * radius, y, angle.sin() * radius]);
        }
    }
    let bottom_index = vertices.len() as u32;
    vertices.push([0.0, 0.0, 0.0]);

    let mut indices = Vec::new();
    // Top cap.
    for segment in 0..radial_segments {
        let next = (segment + 1) % radial_segments;
        indices.extend_from_slice(&[0, (1 + segment) as u32, (1 + next) as u32]);
    }

    // Side quads split into triangles.
    for layer in 0..(vertical_layers - 1) {
        let current_start = 1 + layer * radial_segments;
        let next_start = current_start + radial_segments;
        for segment in 0..radial_segments {
            let next_segment = (segment + 1) % radial_segments;
            let a = (current_start + segment) as u32;
            let b = (current_start + next_segment) as u32;
            let c = (next_start + segment) as u32;
            let d = (next_start + next_segment) as u32;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    // Bottom cap.
    let bottom_ring_start = 1 + (vertical_layers - 1) * radial_segments;
    for segment in 0..radial_segments {
        let next = (segment + 1) % radial_segments;
        indices.extend_from_slice(&[
            bottom_index,
            (bottom_ring_start + next) as u32,
            (bottom_ring_start + segment) as u32,
        ]);
    }

    let bounds = calculate_bounds(&vertices);
    let harvest_points =
        generate_harvest_points(seed, config.harvest_point_count.max(1), &vertices, bounds);

    RockAssembly {
        vertices,
        indices,
        bounds,
        harvest_points,
    }
}

fn generate_harvest_points(
    seed: WorldSeed,
    harvest_point_count: usize,
    vertices: &[[f32; 3]],
    bounds: RockBounds,
) -> Vec<RockHarvestPoint> {
    (0..harvest_point_count)
        .map(|index| {
            let vertex_index = 1
                + (seed
                    .derive_child("rock.harvest.vertex", &[], index as u64)
                    .as_u64() as usize
                    % (vertices.len() - 2));
            let position = clamp_to_bounds(vertices[vertex_index], bounds);
            RockHarvestPoint {
                position,
                normal: normalize([
                    position[0],
                    position[1] - bounds.height() * 0.45,
                    position[2],
                ]),
            }
        })
        .collect()
}

fn calculate_bounds(vertices: &[[f32; 3]]) -> RockBounds {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for vertex in vertices {
        for axis in 0..3 {
            min[axis] = min[axis].min(vertex[axis]);
            max[axis] = max[axis].max(vertex[axis]);
        }
    }
    RockBounds { min, max }
}

fn clamp_to_bounds(mut position: [f32; 3], bounds: RockBounds) -> [f32; 3] {
    for (axis, coordinate) in position.iter_mut().enumerate() {
        *coordinate = coordinate.clamp(bounds.min[axis], bounds.max[axis]);
    }
    position
}

fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    if length == 0.0 {
        [0.0, 1.0, 0.0]
    } else {
        [vector[0] / length, vector[1] / length, vector[2] / length]
    }
}

fn quantize(value: f32) -> i32 {
    (value * 1000.0).round() as i32
}

fn signed_unit(seed: WorldSeed, label: &str, index: u64) -> f32 {
    unit(seed, label, index) * 2.0 - 1.0
}

fn unit(seed: WorldSeed, label: &str, index: u64) -> f32 {
    let child = seed.derive_child(label, &[], index).as_u64();
    let unit = (child >> 11) as f64 / ((1_u64 << 53) as f64);
    unit as f32
}
