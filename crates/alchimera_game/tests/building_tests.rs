use alchimera_core::{ids::ItemId, seed::WorldSeed};
use alchimera_game::{
    building::{BuildPlacementState, BuildingPlugin, PlaceObject, PlayerBuiltObject},
    harvesting::PlayerInventory,
    objects::WorldObject,
};
use alchimera_generation::{chunk::ChunkCoord, objects::ObjectPrototypeKey};
use bevy::prelude::{App, MinimalPlugins, Transform, Vec3};

#[test]
fn place_object_event_spawns_player_object() {
    let mut app = building_app(WorldSeed::new(0xB111D));
    let item = item_id("alchimera:workbench_item");
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(item.clone(), 1, 64);

    app.world_mut().send_event(PlaceObject::new(
        ObjectPrototypeKey::Boulder,
        item,
        Transform::from_translation(Vec3::new(3.0, 0.0, 4.0)),
        ChunkCoord::new(0, 0),
    ));
    app.update();

    let mut built = app.world_mut().query::<&PlayerBuiltObject>();
    assert_eq!(built.iter(app.world()).count(), 1);
}

#[test]
fn placement_consumes_inventory_item() {
    let mut app = building_app(WorldSeed::new(0xB111D));
    let item = item_id("alchimera:workbench_item");
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(item.clone(), 2, 64);

    app.world_mut().send_event(PlaceObject::new(
        ObjectPrototypeKey::Boulder,
        item.clone(),
        Transform::default(),
        ChunkCoord::new(0, 0),
    ));
    app.update();

    assert_eq!(
        app.world()
            .resource::<PlayerInventory>()
            .total_quantity(&item),
        1
    );
}

#[test]
fn placed_object_receives_stable_player_object_id() {
    let seed = WorldSeed::new(0xB111D);
    let chunk = ChunkCoord::new(2, -3);
    let mut app = building_app(seed);
    let item = item_id("alchimera:workbench_item");
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(item.clone(), 1, 64);

    app.world_mut().send_event(PlaceObject::new(
        ObjectPrototypeKey::Tree,
        item,
        Transform::from_translation(Vec3::new(1.0, 2.0, 3.0)),
        chunk,
    ));
    app.update();

    let mut query = app.world_mut().query::<(&WorldObject, &Transform)>();
    let (object, transform) = query.single(app.world());
    assert_eq!(
        object.id,
        BuildPlacementState::new(seed).next_object_id_for_chunk(chunk)
    );
    assert_eq!(object.prototype_key, ObjectPrototypeKey::Tree);
    assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
}

fn building_app(seed: WorldSeed) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BuildingPlugin));
    app.insert_resource(BuildPlacementState::new(seed));
    app
}

fn item_id(value: &str) -> ItemId {
    ItemId::new(value).unwrap()
}
