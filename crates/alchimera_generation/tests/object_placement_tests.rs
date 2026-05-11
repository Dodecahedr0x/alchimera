use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::{ChunkCoord, CHUNK_SIZE_METERS},
    objects::{ObjectGenerator, ObjectPlacementConfig, ProceduralGameObject},
    terrain::TerrainConfig,
};

#[test]
fn same_seed_and_chunk_generate_same_object_instances() {
    let generator = ObjectGenerator::default();
    let seed = WorldSeed::new(0x0B1EC7);
    let chunk = ChunkCoord::new(4, -2);

    let first = generator.generate_chunk(seed, chunk);
    let second = generator.generate_chunk(seed, chunk);

    let first_summary: Vec<_> = first
        .iter()
        .map(|object| (object.id(), object.prototype_key(), object.transform()))
        .collect();
    let second_summary: Vec<_> = second
        .iter()
        .map(|object| (object.id(), object.prototype_key(), object.transform()))
        .collect();

    assert_eq!(first_summary, second_summary);
}

#[test]
fn different_seed_changes_object_summary() {
    let generator = ObjectGenerator::default();
    let chunk = ChunkCoord::new(0, 0);

    let first_summary: Vec<_> = generator
        .generate_chunk(WorldSeed::new(1), chunk)
        .into_iter()
        .map(|object| (object.id(), object.prototype_key(), object.transform()))
        .collect();
    let second_summary: Vec<_> = generator
        .generate_chunk(WorldSeed::new(2), chunk)
        .into_iter()
        .map(|object| (object.id(), object.prototype_key(), object.transform()))
        .collect();

    assert_ne!(first_summary, second_summary);
}

#[test]
fn objects_are_within_chunk_bounds() {
    let generator = ObjectGenerator::default();
    let chunk = ChunkCoord::new(-3, 5);
    let bounds = chunk.world_bounds();

    for object in generator.generate_chunk(WorldSeed::new(7), chunk) {
        let [x, _y, z] = object.transform().translation;
        assert!((bounds.min_x..bounds.max_x).contains(&x));
        assert!((bounds.min_z..bounds.max_z).contains(&z));
        let (local_x, local_z) = object.local_position();
        assert!((0.0..CHUNK_SIZE_METERS).contains(&local_x));
        assert!((0.0..CHUNK_SIZE_METERS).contains(&local_z));
    }
}

#[test]
fn objects_do_not_spawn_on_invalid_slope_when_slope_filter_enabled() {
    let terrain_config = TerrainConfig::new(-128.0, 128.0);
    let unfiltered = ObjectGenerator::with_placement_config(
        32,
        terrain_config,
        ObjectPlacementConfig::default().with_max_slope_height_delta(f32::INFINITY),
    )
    .generate_chunk(WorldSeed::new(42), ChunkCoord::new(0, 0));
    let placement_config = ObjectPlacementConfig::default().with_max_slope_height_delta(0.0);
    let filtered = ObjectGenerator::with_placement_config(32, terrain_config, placement_config)
        .generate_chunk(WorldSeed::new(42), ChunkCoord::new(0, 0));

    assert!(filtered.len() < unfiltered.len());
    assert!(filtered
        .iter()
        .all(|object| object.slope_height_delta() <= placement_config.max_slope_height_delta()));
}
