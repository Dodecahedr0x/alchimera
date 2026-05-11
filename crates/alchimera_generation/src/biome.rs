//! Deterministic biome sampling for initial terrain generation.

use alchimera_core::seed::WorldSeed;

use crate::chunk::ChunkCoord;

/// Initial prototype biomes available to the terrain and object generators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    Grassland,
    Forest,
    RockyHighland,
    RiverValley,
}

impl Biome {
    #[must_use]
    pub const fn initial_biomes() -> &'static [Self] {
        &[
            Self::Grassland,
            Self::Forest,
            Self::RockyHighland,
            Self::RiverValley,
        ]
    }
}

/// Stateless deterministic biome sampler.
#[derive(Debug, Default, Clone, Copy)]
pub struct BiomeSampler;

impl BiomeSampler {
    #[must_use]
    pub fn sample(self, seed: WorldSeed, chunk: ChunkCoord, local_x: f32, local_z: f32) -> Biome {
        sample_biome(seed, chunk, local_x, local_z)
    }
}

#[must_use]
pub fn sample_biome(seed: WorldSeed, chunk: ChunkCoord, local_x: f32, local_z: f32) -> Biome {
    let world_x = chunk.x as f32 * crate::chunk::CHUNK_SIZE_METERS + local_x;
    let world_z = chunk.z as f32 * crate::chunk::CHUNK_SIZE_METERS + local_z;
    let valley_band = (world_x - world_z).abs();
    let valley_noise = signed_noise(seed, "biome.river", world_x, world_z).abs();

    if valley_band <= 4.0 || valley_noise <= 0.08 {
        return Biome::RiverValley;
    }

    let biome_noise = signed_noise(seed, "biome.primary", world_x * 0.5, world_z * 0.5);
    if biome_noise < -0.25 {
        Biome::Grassland
    } else if biome_noise < 0.35 {
        Biome::Forest
    } else {
        Biome::RockyHighland
    }
}

fn signed_noise(seed: WorldSeed, label: &str, world_x: f32, world_z: f32) -> f32 {
    let quantized_x = (world_x / 8.0).floor() as i32;
    let quantized_z = (world_z / 8.0).floor() as i32;
    let child = seed
        .derive_child(label, &[quantized_x, quantized_z], 0)
        .as_u64();
    let unit = (child >> 11) as f64 / ((1_u64 << 53) as f64);
    (unit.mul_add(2.0, -1.0)) as f32
}
