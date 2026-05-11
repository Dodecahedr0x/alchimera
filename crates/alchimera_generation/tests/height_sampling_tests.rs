use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    terrain::{sample_height, HeightSampler, TerrainConfig},
};

#[test]
fn same_seed_same_position_same_height() {
    let sampler = HeightSampler::new(TerrainConfig::default());
    let seed = WorldSeed::new(123);
    let chunk = ChunkCoord::new(-2, 5);

    assert_eq!(
        sampler.sample(seed, chunk, 8.0, 24.0),
        sampler.sample(seed, chunk, 8.0, 24.0)
    );
}

#[test]
fn different_seed_changes_height_summary() {
    let config = TerrainConfig::default();
    let sampler = HeightSampler::new(config);
    let chunk = ChunkCoord::new(1, 1);

    let first: Vec<f32> = (0..8)
        .map(|i| sampler.sample(WorldSeed::new(1), chunk, i as f32 * 4.0, 12.0))
        .collect();
    let second: Vec<f32> = (0..8)
        .map(|i| sampler.sample(WorldSeed::new(2), chunk, i as f32 * 4.0, 12.0))
        .collect();

    assert_ne!(first, second);
}

#[test]
fn height_values_stay_within_configured_bounds() {
    let config = TerrainConfig::new(-12.0, 48.0);

    for x in -4..=4 {
        for z in -4..=4 {
            let height = sample_height(
                WorldSeed::new(77),
                ChunkCoord::new(x, z),
                32.0,
                48.0,
                config,
            );
            assert!(height >= config.min_height(), "height {height} below min");
            assert!(height <= config.max_height(), "height {height} above max");
        }
    }
}
