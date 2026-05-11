use alchimera_core::seed::WorldSeed;
use alchimera_generation::tree::{generate_tree, TreeConfig};

#[test]
fn same_tree_seed_generates_same_summary() {
    let seed = WorldSeed::new(42);
    let first = generate_tree(seed, TreeConfig::default()).summary();
    let second = generate_tree(seed, TreeConfig::default()).summary();

    assert_eq!(first, second);
}

#[test]
fn tree_has_trunk_and_leaf_clusters() {
    let tree = generate_tree(WorldSeed::new(7), TreeConfig::default());

    assert!(!tree.trunk_segments.is_empty());
    assert!(!tree.branches.is_empty());
    assert!(!tree.leaf_clusters.is_empty());
}

#[test]
fn tree_attachment_points_are_stable() {
    let seed = WorldSeed::new(99);
    let first = generate_tree(seed, TreeConfig::default()).attachment_points;
    let second = generate_tree(seed, TreeConfig::default()).attachment_points;

    assert!(!first.is_empty());
    assert_eq!(first, second);
}
