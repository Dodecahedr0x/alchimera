use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    biome::{sample_biome, Biome, BiomeSampler},
    chunk::ChunkCoord,
};

#[test]
fn same_seed_position_returns_same_biome() {
    let sampler = BiomeSampler;
    let seed = WorldSeed::new(42);
    let chunk = ChunkCoord::new(3, -2);

    assert_eq!(
        sampler.sample(seed, chunk, 12.5, 44.0),
        sampler.sample(seed, chunk, 12.5, 44.0)
    );
}

#[test]
fn biome_selection_only_returns_initial_biomes() {
    let sampler = BiomeSampler;
    let seed = WorldSeed::new(7);

    for x in -8..=8 {
        for z in -8..=8 {
            let biome = sampler.sample(seed, ChunkCoord::new(x, z), 16.0, 16.0);
            assert!(Biome::initial_biomes().contains(&biome));
        }
    }
}

#[test]
fn river_valley_can_be_selected_near_low_noise_band() {
    let biome = sample_biome(WorldSeed::new(0), ChunkCoord::new(0, 0), 0.0, 0.0);

    assert_eq!(biome, Biome::RiverValley);
}
