use alchimera_core::ids::ObjectId;
use alchimera_game::{
    interaction::{CurrentInteractionTarget, InteractionRaycast},
    objects::WorldObject,
};
use alchimera_generation::objects::ObjectPrototypeKey;
use bevy::prelude::{App, MinimalPlugins};

#[test]
fn raycast_hit_updates_current_interaction_target() {
    let mut app = interaction_app();
    let object_id = ObjectId::from_seed_chunk_and_index(WorldSeedFixture::seed(), [0, 0], 0);
    let entity = app
        .world_mut()
        .spawn((WorldObject {
            id: object_id,
            prototype_key: ObjectPrototypeKey::Tree,
        },))
        .id();

    app.world_mut().send_event(InteractionRaycast::Hit(entity));
    app.update();

    let current = app.world().resource::<CurrentInteractionTarget>();
    assert_eq!(current.entity(), Some(entity));
    assert_eq!(current.object_id(), Some(object_id));
}

#[test]
fn no_hit_clears_current_interaction_target() {
    let mut app = interaction_app();
    let object_id = ObjectId::from_seed_chunk_and_index(WorldSeedFixture::seed(), [1, 0], 0);
    let entity = app
        .world_mut()
        .spawn((WorldObject {
            id: object_id,
            prototype_key: ObjectPrototypeKey::Boulder,
        },))
        .id();

    app.world_mut().send_event(InteractionRaycast::Hit(entity));
    app.update();
    app.world_mut().send_event(InteractionRaycast::Miss);
    app.update();

    let current = app.world().resource::<CurrentInteractionTarget>();
    assert_eq!(current.entity(), None);
    assert_eq!(current.object_id(), None);
}

fn interaction_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        alchimera_game::interaction::InteractionPlugin,
    ));
    app
}

struct WorldSeedFixture;

impl WorldSeedFixture {
    fn seed() -> alchimera_core::seed::WorldSeed {
        alchimera_core::seed::WorldSeed::new(0x1A7E)
    }
}
