use alchimera_core::{
    ids::{ItemId, ObjectId},
    seed::WorldSeed,
};
use alchimera_game::{
    building::PlayerBuiltObject,
    harvesting::{ChunkModificationLogs, PlayerInventory},
    objects::WorldObject,
    persistence::{load_game_state, save_game_state, PersistentWorldSeed},
};
use alchimera_generation::{chunk::ChunkCoord, objects::ObjectPrototypeKey};
use bevy::prelude::{App, MinimalPlugins, Transform, Vec3};

#[test]
fn saving_and_loading_preserves_inventory() {
    let mut app = persistence_app(WorldSeed::new(0x51A7E));
    let wood = ItemId::new("material.wood").unwrap();
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(wood.clone(), 7, 64);

    let save = save_game_state(app.world_mut());
    let mut loaded = persistence_app(WorldSeed::new(0));
    load_game_state(loaded.world_mut(), save);

    assert_eq!(
        loaded
            .world()
            .resource::<PlayerInventory>()
            .total_quantity(&wood),
        7
    );
    assert_eq!(
        loaded.world().resource::<PersistentWorldSeed>().seed(),
        WorldSeed::new(0x51A7E)
    );
}

#[test]
fn saving_and_loading_preserves_harvested_object_override() {
    let mut app = persistence_app(WorldSeed::new(44));
    let chunk = ChunkCoord::new(-2, 5);
    let object_id = ObjectId::from_seed_chunk_and_index(WorldSeed::new(44), [chunk.x, chunk.z], 3);
    app.world_mut()
        .resource_mut::<ChunkModificationLogs>()
        .log_for_mut(chunk)
        .record_removed(object_id);

    let save = save_game_state(app.world_mut());
    let mut loaded = persistence_app(WorldSeed::new(0));
    load_game_state(loaded.world_mut(), save);

    assert!(loaded
        .world()
        .resource::<ChunkModificationLogs>()
        .log_for(chunk)
        .unwrap()
        .is_removed(object_id));
}

#[test]
fn saving_and_loading_preserves_placed_object() {
    let mut app = persistence_app(WorldSeed::new(99));
    let chunk = ChunkCoord::new(1, -1);
    let object_id = ObjectId::from_seed_chunk_and_index(WorldSeed::new(99), [chunk.x, chunk.z], 8);
    app.world_mut().spawn((
        WorldObject {
            id: object_id,
            prototype_key: ObjectPrototypeKey::Boulder,
        },
        PlayerBuiltObject { chunk },
        Transform::from_translation(Vec3::new(3.0, 4.0, 5.0)),
    ));

    let save = save_game_state(app.world_mut());
    let mut loaded = persistence_app(WorldSeed::new(0));
    load_game_state(loaded.world_mut(), save);

    let mut query = loaded
        .world_mut()
        .query::<(&WorldObject, &PlayerBuiltObject, &Transform)>();
    let placed = query.single(loaded.world());
    assert_eq!(placed.0.id, object_id);
    assert_eq!(placed.0.prototype_key, ObjectPrototypeKey::Boulder);
    assert_eq!(placed.1.chunk, chunk);
    assert_eq!(placed.2.translation.to_array(), [3.0, 4.0, 5.0]);
}

fn persistence_app(seed: WorldSeed) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(PersistentWorldSeed::new(seed));
    app.insert_resource(PlayerInventory::default());
    app.insert_resource(ChunkModificationLogs::default());
    app
}
