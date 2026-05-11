//! Deterministic prototype terrain height sampling.

use alchimera_core::seed::WorldSeed;

use crate::chunk::{ChunkCoord, CHUNK_SIZE_METERS};

/// Height sampler configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainConfig {
    min_height: f32,
    max_height: f32,
}

impl TerrainConfig {
    #[must_use]
    pub const fn new(min_height: f32, max_height: f32) -> Self {
        Self {
            min_height,
            max_height,
        }
    }

    #[must_use]
    pub const fn min_height(self) -> f32 {
        self.min_height
    }

    #[must_use]
    pub const fn max_height(self) -> f32 {
        self.max_height
    }
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self::new(-8.0, 36.0)
    }
}

/// Stateless deterministic height sampler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeightSampler {
    config: TerrainConfig,
}

impl HeightSampler {
    #[must_use]
    pub const fn new(config: TerrainConfig) -> Self {
        Self { config }
    }

    #[must_use]
    pub fn sample(self, seed: WorldSeed, chunk: ChunkCoord, local_x: f32, local_z: f32) -> f32 {
        sample_height(seed, chunk, local_x, local_z, self.config)
    }
}

#[must_use]
pub fn sample_height(
    seed: WorldSeed,
    chunk: ChunkCoord,
    local_x: f32,
    local_z: f32,
    config: TerrainConfig,
) -> f32 {
    let world_x = chunk.x as f32 * CHUNK_SIZE_METERS + local_x;
    let world_z = chunk.z as f32 * CHUNK_SIZE_METERS + local_z;
    let broad = signed_noise(seed, "terrain.height.broad", world_x / 32.0, world_z / 32.0);
    let detail = signed_noise(seed, "terrain.height.detail", world_x / 8.0, world_z / 8.0) * 0.25;
    let normalized = ((broad + detail + 1.25) / 2.5).clamp(0.0, 1.0);
    config.min_height + normalized * (config.max_height - config.min_height)
}

fn signed_noise(seed: WorldSeed, label: &str, x: f32, z: f32) -> f32 {
    let qx = x.floor() as i32;
    let qz = z.floor() as i32;
    let hash = seed.derive_child(label, &[qx, qz], 0).as_u64();
    let unit = (hash >> 11) as f64 / ((1_u64 << 53) as f64);
    (unit.mul_add(2.0, -1.0)) as f32
}
