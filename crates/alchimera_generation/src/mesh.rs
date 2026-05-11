//! Pure terrain mesh data generation.

use alchimera_core::seed::WorldSeed;

use crate::{
    chunk::{ChunkCoord, CHUNK_SIZE_METERS},
    terrain::{sample_height, TerrainConfig},
};

/// Configuration for heightmap terrain mesh generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainMeshConfig {
    subdivisions: u32,
}

impl TerrainMeshConfig {
    #[must_use]
    pub const fn new(subdivisions: u32) -> Self {
        Self { subdivisions }
    }

    #[must_use]
    pub const fn subdivisions(self) -> u32 {
        self.subdivisions
    }
}

/// Engine-agnostic terrain mesh payload.
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainMeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl TerrainMeshData {
    #[must_use]
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    #[must_use]
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    #[must_use]
    pub fn uvs(&self) -> &[[f32; 2]] {
        &self.uvs
    }

    #[must_use]
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

/// Stateless generator for deterministic chunk terrain mesh data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainMeshGenerator {
    terrain_config: TerrainConfig,
    mesh_config: TerrainMeshConfig,
}

impl TerrainMeshGenerator {
    #[must_use]
    pub const fn new(terrain_config: TerrainConfig, mesh_config: TerrainMeshConfig) -> Self {
        Self {
            terrain_config,
            mesh_config,
        }
    }

    #[must_use]
    pub fn generate_chunk(self, seed: WorldSeed, chunk: ChunkCoord) -> TerrainMeshData {
        generate_terrain_mesh(seed, chunk, self.terrain_config, self.mesh_config)
    }
}

#[must_use]
pub fn generate_terrain_mesh(
    seed: WorldSeed,
    chunk: ChunkCoord,
    terrain_config: TerrainConfig,
    mesh_config: TerrainMeshConfig,
) -> TerrainMeshData {
    let subdivisions = mesh_config.subdivisions.max(1);
    let vertices_per_axis = subdivisions + 1;
    let vertex_count = (vertices_per_axis * vertices_per_axis) as usize;
    let quad_count = subdivisions * subdivisions;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let step = CHUNK_SIZE_METERS / subdivisions as f32;

    for z in 0..=subdivisions {
        for x in 0..=subdivisions {
            let local_x = x as f32 * step;
            let local_z = z as f32 * step;
            let height = sample_height(seed, chunk, local_x, local_z, terrain_config);
            positions.push([local_x, height, local_z]);
            uvs.push([
                x as f32 / subdivisions as f32,
                z as f32 / subdivisions as f32,
            ]);
        }
    }

    let mut indices = Vec::with_capacity((quad_count * 6) as usize);
    for z in 0..subdivisions {
        for x in 0..subdivisions {
            let top_left = z * vertices_per_axis + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + vertices_per_axis;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    let normals = calculate_normals(&positions, subdivisions);

    TerrainMeshData {
        positions,
        normals,
        uvs,
        indices,
    }
}

fn calculate_normals(positions: &[[f32; 3]], subdivisions: u32) -> Vec<[f32; 3]> {
    let vertices_per_axis = subdivisions + 1;
    let mut normals = Vec::with_capacity(positions.len());

    for z in 0..=subdivisions {
        for x in 0..=subdivisions {
            let center = height_at(positions, vertices_per_axis, x, z);
            let left = height_at(positions, vertices_per_axis, x.saturating_sub(1), z);
            let right = height_at(positions, vertices_per_axis, (x + 1).min(subdivisions), z);
            let down = height_at(positions, vertices_per_axis, x, z.saturating_sub(1));
            let up = height_at(positions, vertices_per_axis, x, (z + 1).min(subdivisions));

            let dx = if x == 0 {
                right - center
            } else if x == subdivisions {
                center - left
            } else {
                (right - left) * 0.5
            };
            let dz = if z == 0 {
                up - center
            } else if z == subdivisions {
                center - down
            } else {
                (up - down) * 0.5
            };
            normals.push(normalize([-dx, 1.0, -dz]));
        }
    }

    normals
}

fn height_at(positions: &[[f32; 3]], vertices_per_axis: u32, x: u32, z: u32) -> f32 {
    positions[(z * vertices_per_axis + x) as usize][1]
}

fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let length = (vector[0].powi(2) + vector[1].powi(2) + vector[2].powi(2)).sqrt();
    [vector[0] / length, vector[1] / length, vector[2] / length]
}
