use alchimera_core::seed::WorldSeed;
use alchimera_generation::rock::{generate_rock, RockConfig};

#[test]
fn same_rock_seed_generates_same_summary() {
    let seed = WorldSeed::new(31337);
    let first = generate_rock(seed, RockConfig::default()).summary();
    let second = generate_rock(seed, RockConfig::default()).summary();

    assert_eq!(first, second);
}

#[test]
fn rock_has_nonzero_bounds() {
    let rock = generate_rock(WorldSeed::new(23), RockConfig::default());
    let bounds = rock.bounds;

    assert!(bounds.width() > 0.0);
    assert!(bounds.height() > 0.0);
    assert!(bounds.depth() > 0.0);
    assert!(!rock.vertices.is_empty());
}

#[test]
fn rock_harvest_points_are_within_bounds() {
    let rock = generate_rock(WorldSeed::new(404), RockConfig::default());

    assert!(!rock.harvest_points.is_empty());
    for point in &rock.harvest_points {
        assert!(rock.bounds.contains(point.position));
    }
}
