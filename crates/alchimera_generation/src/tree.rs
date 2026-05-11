//! Pure deterministic tree assembly data.

use alchimera_core::seed::WorldSeed;

/// Configuration for procedural tree assembly generation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeConfig {
    pub trunk_segments: usize,
    pub branch_count: usize,
    pub leaf_cluster_count: usize,
    pub base_height: f32,
    pub height_variation: f32,
    pub trunk_radius: f32,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            trunk_segments: 5,
            branch_count: 4,
            leaf_cluster_count: 5,
            base_height: 5.0,
            height_variation: 2.0,
            trunk_radius: 0.28,
        }
    }
}

/// A vertical trunk segment represented as a simple cylinder source.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrunkSegment {
    pub base: [f32; 3],
    pub top: [f32; 3],
    pub radius: f32,
}

/// A branch segment extending from the trunk to a stable attachment point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BranchSegment {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub radius: f32,
}

/// A leaf cluster represented by a center point and radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafCluster {
    pub center: [f32; 3],
    pub radius: f32,
}

/// Semantic attachment target for later object-spawning and harvesting adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttachmentKind {
    Branch,
    LeafCluster,
    CanopyTop,
}

/// Stable point generated as an adapter hook for gameplay and rendering systems.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeAttachmentPoint {
    pub kind: AttachmentKind,
    pub position: [f32; 3],
}

/// Pure generated tree assembly data.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeAssembly {
    pub trunk_segments: Vec<TrunkSegment>,
    pub branches: Vec<BranchSegment>,
    pub leaf_clusters: Vec<LeafCluster>,
    pub attachment_points: Vec<TreeAttachmentPoint>,
}

impl TreeAssembly {
    /// Returns a compact deterministic summary for tests, cache keys, and diagnostics.
    #[must_use]
    pub fn summary(&self) -> TreeSummary {
        let height_millimeters = self
            .trunk_segments
            .last()
            .map_or(0, |segment| quantize(segment.top[1]));
        let branch_reach_millimeters = self
            .branches
            .iter()
            .map(|branch| quantize(distance_xz(branch.start, branch.end)))
            .sum();
        let leaf_radius_millimeters = self
            .leaf_clusters
            .iter()
            .map(|cluster| quantize(cluster.radius))
            .sum();

        TreeSummary {
            trunk_segments: self.trunk_segments.len(),
            branches: self.branches.len(),
            leaf_clusters: self.leaf_clusters.len(),
            attachment_points: self.attachment_points.len(),
            height_millimeters,
            branch_reach_millimeters,
            leaf_radius_millimeters,
        }
    }
}

/// Compact deterministic tree summary that avoids snapshotting whole meshes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeSummary {
    pub trunk_segments: usize,
    pub branches: usize,
    pub leaf_clusters: usize,
    pub attachment_points: usize,
    pub height_millimeters: i32,
    pub branch_reach_millimeters: i32,
    pub leaf_radius_millimeters: i32,
}

/// Generates a deterministic tree assembly from a seed and configuration.
#[must_use]
pub fn generate_tree(seed: WorldSeed, config: TreeConfig) -> TreeAssembly {
    let trunk_count = config.trunk_segments.max(1);
    let branch_count = config.branch_count.max(1);
    let leaf_count = config.leaf_cluster_count.max(1);
    let height = config.base_height + unit(seed, "tree.height", 0) * config.height_variation;
    let segment_height = height / trunk_count as f32;

    let trunk_segments = (0..trunk_count)
        .map(|index| {
            let base_y = index as f32 * segment_height;
            let top_y = (index + 1) as f32 * segment_height;
            let taper = 1.0 - (index as f32 / trunk_count as f32) * 0.45;
            TrunkSegment {
                base: [0.0, base_y, 0.0],
                top: [0.0, top_y, 0.0],
                radius: config.trunk_radius * taper,
            }
        })
        .collect::<Vec<_>>();

    let branches = (0..branch_count)
        .map(|index| {
            let fraction = 0.35 + (index as f32 / branch_count as f32) * 0.5;
            let start_y = height * fraction;
            let angle = unit(seed, "tree.branch.angle", index as u64) * std::f32::consts::TAU;
            let reach = 0.9 + unit(seed, "tree.branch.reach", index as u64) * 1.1;
            let lift = 0.25 + unit(seed, "tree.branch.lift", index as u64) * 0.45;
            BranchSegment {
                start: [0.0, start_y, 0.0],
                end: [angle.cos() * reach, start_y + lift, angle.sin() * reach],
                radius: config.trunk_radius * (0.35 + 0.2 * (1.0 - fraction)),
            }
        })
        .collect::<Vec<_>>();

    let leaf_clusters = (0..leaf_count)
        .map(|index| {
            let branch = branches[index % branches.len()];
            let jitter_seed = index as u64;
            LeafCluster {
                center: [
                    branch.end[0] + signed_unit(seed, "tree.leaf.x", jitter_seed) * 0.25,
                    branch.end[1] + 0.15 + unit(seed, "tree.leaf.y", jitter_seed) * 0.35,
                    branch.end[2] + signed_unit(seed, "tree.leaf.z", jitter_seed) * 0.25,
                ],
                radius: 0.55 + unit(seed, "tree.leaf.radius", jitter_seed) * 0.35,
            }
        })
        .collect::<Vec<_>>();

    let mut attachment_points = Vec::with_capacity(branches.len() + leaf_clusters.len() + 1);
    attachment_points.extend(branches.iter().map(|branch| TreeAttachmentPoint {
        kind: AttachmentKind::Branch,
        position: branch.end,
    }));
    attachment_points.extend(leaf_clusters.iter().map(|cluster| TreeAttachmentPoint {
        kind: AttachmentKind::LeafCluster,
        position: cluster.center,
    }));
    attachment_points.push(TreeAttachmentPoint {
        kind: AttachmentKind::CanopyTop,
        position: [0.0, height, 0.0],
    });

    TreeAssembly {
        trunk_segments,
        branches,
        leaf_clusters,
        attachment_points,
    }
}

fn distance_xz(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    (dx * dx + dz * dz).sqrt()
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
