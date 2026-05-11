use alchimera_core::{
    ids::{ItemId, ObjectId},
    seed::WorldSeed,
};
use alchimera_game::{
    harvesting::{
        ChunkModificationLogs, HarvestObject, Harvestable, Harvested, HarvestingPlugin,
        PlayerInventory,
    },
    objects::WorldObject,
};
use alchimera_generation::{chunk::ChunkCoord, objects::ObjectPrototypeKey};
use bevy::prelude::{App, MinimalPlugins};

#[test]
fn harvest_event_adds_yield_to_inventory() {
    let mut app = harvesting_app();
    let item = ItemId::new("material.wood").unwrap();
    let entity = spawn_harvestable(&mut app, item.clone(), ChunkCoord::new(0, 0));

    app.world_mut().send_event(HarvestObject { entity });
    app.update();

    let inventory = app.world().resource::<PlayerInventory>();
    assert_eq!(inventory.total_quantity(&item), 3);
}

#[test]
fn harvest_event_marks_object_harvested() {
    let mut app = harvesting_app();
    let item = ItemId::new("material.stone").unwrap();
    let entity = spawn_harvestable(&mut app, item, ChunkCoord::new(1, 0));

    app.world_mut().send_event(HarvestObject { entity });
    app.update();

    assert!(app.world().entity(entity).contains::<Harvested>());
}

#[test]
fn harvested_generated_object_adds_removed_or_state_override_to_mod_log() {
    let mut app = harvesting_app();
    let item = ItemId::new("material.herb").unwrap();
    let chunk = ChunkCoord::new(-1, 2);
    let entity = spawn_harvestable(&mut app, item, chunk);
    let object_id = app.world().entity(entity).get::<WorldObject>().unwrap().id;

    app.world_mut().send_event(HarvestObject { entity });
    app.update();

    let logs = app.world().resource::<ChunkModificationLogs>();
    assert!(logs.log_for(chunk).unwrap().is_removed(object_id));
}

fn harvesting_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, HarvestingPlugin));
    app
}

fn spawn_harvestable(app: &mut App, item: ItemId, chunk: ChunkCoord) -> bevy::prelude::Entity {
    app.world_mut()
        .spawn((
            WorldObject {
                id: ObjectId::from_seed_chunk_and_index(WorldSeed::new(123), [chunk.x, chunk.z], 0),
                prototype_key: ObjectPrototypeKey::Tree,
            },
            Harvestable {
                yield_item: item,
                quantity: 3,
                stack_limit: 64,
                chunk,
            },
        ))
        .id()
}
