use alchimera_core::ids::ObjectId;
use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    modification_log::{ChunkModificationLog, ObjectPlacementOverride},
};

fn object_id(index: u64) -> ObjectId {
    ObjectId::from_seed_chunk_and_index(WorldSeed::new(7), [0, 0], index)
}

#[test]
fn removed_object_override_hides_generated_object() {
    let removed = object_id(0);
    let kept = object_id(1);
    let mut log = ChunkModificationLog::new(ChunkCoord::new(0, 0));

    log.record_removed(removed);

    assert!(log.is_removed(removed));
    assert!(!log.is_removed(kept));
    assert_eq!(log.visible_generated_objects([removed, kept]), vec![kept]);
}

#[test]
fn placed_object_is_returned_after_applying_log() {
    let generated = object_id(0);
    let placed = ObjectPlacementOverride::new("player_workbench", "workbench", [12.0, 3.0, 8.0]);
    let mut log = ChunkModificationLog::new(ChunkCoord::new(0, 0));

    log.record_placed(placed.clone());

    let resolved = log.apply_to_generated([generated]);
    assert_eq!(resolved.generated_object_ids, vec![generated]);
    assert_eq!(resolved.placed_objects, vec![placed]);
}

#[test]
fn modification_log_roundtrips_through_save_format() {
    let mut log = ChunkModificationLog::new(ChunkCoord::new(-2, 3));
    log.record_removed(object_id(0));
    log.record_damaged(object_id(1), 25);
    log.record_placed(ObjectPlacementOverride::new(
        "player_wall_1",
        "wood_wall",
        [1.0, 2.0, 3.0],
    ));

    let encoded = log.to_save_json().expect("serialize modification log");
    let decoded =
        ChunkModificationLog::from_save_json(&encoded).expect("deserialize modification log");

    assert_eq!(decoded, log);
    assert!(decoded.is_removed(object_id(0)));
    assert_eq!(decoded.damage_for(object_id(1)), Some(25));
}
