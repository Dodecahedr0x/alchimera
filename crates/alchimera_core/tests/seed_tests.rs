use alchimera_core::seed::WorldSeed;

#[test]
fn same_inputs_derive_same_child_seed() {
    let seed = WorldSeed::new(12_345);

    let a = seed.derive_child("chunk.objects", &[4, -2], 17);
    let b = seed.derive_child("chunk.objects", &[4, -2], 17);

    assert_eq!(a, b);
}

#[test]
fn different_labels_derive_different_child_seeds() {
    let seed = WorldSeed::new(12_345);

    let objects = seed.derive_child("chunk.objects", &[4, -2], 17);
    let terrain = seed.derive_child("chunk.terrain", &[4, -2], 17);

    assert_ne!(objects, terrain);
}

#[test]
fn different_indices_derive_different_child_seeds() {
    let seed = WorldSeed::new(12_345);

    let first = seed.derive_child("chunk.objects", &[4, -2], 17);
    let second = seed.derive_child("chunk.objects", &[4, -2], 18);

    assert_ne!(first, second);
}

#[test]
fn child_seed_can_be_used_as_u64() {
    let seed = WorldSeed::new(12_345);

    let child = seed.derive_child("chunk.objects", &[0, 0], 0);

    assert_ne!(child.as_u64(), 0);
}
