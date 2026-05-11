use alchimera_core::save::{SaveData, SaveError, CURRENT_SAVE_VERSION};

#[test]
fn new_save_data_has_current_version() {
    let save = SaveData::new("world.seed.demo");

    assert_eq!(save.version(), CURRENT_SAVE_VERSION);
    assert_eq!(save.world_seed_label(), "world.seed.demo");
}

#[test]
fn save_data_roundtrips_through_json() {
    let save = SaveData::new("world.seed.demo");

    let encoded = serde_json::to_string(&save).expect("save should serialize");
    let decoded: SaveData = serde_json::from_str(&encoded).expect("save should deserialize");

    assert_eq!(decoded, save);
}

#[test]
fn unsupported_version_returns_error() {
    let save = SaveData::with_version_for_testing(CURRENT_SAVE_VERSION + 1, "future.seed");

    assert_eq!(
        save.validate_version(),
        Err(SaveError::UnsupportedVersion {
            found: CURRENT_SAVE_VERSION + 1,
            supported: CURRENT_SAVE_VERSION,
        })
    );
}
