use alchimera_core::{
    ids::{MaterialId, ObjectId, PrototypeId},
    seed::WorldSeed,
};

#[test]
fn id_prototype_id_rejects_empty_string() {
    assert!(PrototypeId::new("").is_err());
    assert!(PrototypeId::new("   ").is_err());
}

#[test]
fn id_material_id_preserves_namespaced_value() {
    let id = MaterialId::new("wood.oak").expect("valid material id");

    assert_eq!(id.as_str(), "wood.oak");
    assert_eq!(id.to_string(), "wood.oak");
}

#[test]
fn id_object_id_is_stable_from_seed_chunk_and_index() {
    let seed = WorldSeed::new(42);

    let first = ObjectId::from_seed_chunk_and_index(seed, [3, -7], 11);
    let second = ObjectId::from_seed_chunk_and_index(seed, [3, -7], 11);
    let different_index = ObjectId::from_seed_chunk_and_index(seed, [3, -7], 12);

    assert_eq!(first, second);
    assert_ne!(first, different_index);
}
