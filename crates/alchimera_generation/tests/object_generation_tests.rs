use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    objects::{
        object_catalog, LifecycleState, ObjectGenerator, ObjectPrototypeKey, ProceduralGameObject,
    },
};

#[test]
fn objects_are_generated_deterministically_from_seed_and_chunk() {
    let generator = ObjectGenerator::default();
    let seed = WorldSeed::new(0xA1C0_E011);
    let chunk = ChunkCoord::new(2, -3);

    let first = generator.generate_chunk(seed, chunk);
    let second = generator.generate_chunk(seed, chunk);

    assert!(
        !first.is_empty(),
        "chunks should contain procedural objects"
    );
    assert_eq!(first, second);
    assert!(first.iter().all(|object| object.chunk() == chunk));
}

#[test]
fn object_identity_changes_when_procedural_inputs_change() {
    let generator = ObjectGenerator::default();
    let chunk = ChunkCoord::new(0, 0);

    let baseline = generator.generate_chunk(WorldSeed::new(7), chunk);
    let changed_seed = generator.generate_chunk(WorldSeed::new(8), chunk);
    let changed_chunk = generator.generate_chunk(WorldSeed::new(7), ChunkCoord::new(1, 0));

    let baseline_ids: Vec<_> = baseline.iter().map(|object| object.id()).collect();
    let changed_seed_ids: Vec<_> = changed_seed.iter().map(|object| object.id()).collect();
    let changed_chunk_ids: Vec<_> = changed_chunk.iter().map(|object| object.id()).collect();

    assert_ne!(baseline_ids, changed_seed_ids);
    assert_ne!(baseline_ids, changed_chunk_ids);
}

#[test]
fn every_catalog_object_has_a_procedural_visualization_key() {
    let catalog = object_catalog();

    assert!(
        catalog.len() >= 3,
        "need multiple object archetypes to visualize"
    );
    for prototype in catalog {
        assert!(!prototype.key.as_str().is_empty());
        assert!(!prototype.display_name.is_empty());
        assert!(!prototype.ascii_icon.is_empty());
        assert!(prototype.spawn_weight > 0);
    }
}

#[test]
fn generated_objects_implement_common_lifecycle_trait() {
    let generator = ObjectGenerator::default();
    let mut object = generator
        .generate_chunk(WorldSeed::new(99), ChunkCoord::new(0, 0))
        .into_iter()
        .next()
        .expect("object generated");

    assert_eq!(object.lifecycle_state(), LifecycleState::Procedural);

    let created = object.create();
    assert_eq!(created.object_id, object.id());
    assert_eq!(created.new_state, LifecycleState::Created);
    assert_eq!(object.lifecycle_state(), LifecycleState::Created);

    let activated = object.activate();
    assert_eq!(activated.new_state, LifecycleState::Active);
    assert_eq!(object.lifecycle_state(), LifecycleState::Active);

    let destroyed = object.destroy();
    assert_eq!(destroyed.new_state, LifecycleState::Destroyed);
    assert_eq!(object.lifecycle_state(), LifecycleState::Destroyed);
}

#[test]
fn prototype_keys_are_stable_for_visualizer_output() {
    let keys: Vec<_> = object_catalog()
        .iter()
        .map(|prototype| prototype.key)
        .collect();

    assert!(keys.contains(&ObjectPrototypeKey::Tree));
    assert!(keys.contains(&ObjectPrototypeKey::Boulder));
    assert!(keys.contains(&ObjectPrototypeKey::Herb));
}
