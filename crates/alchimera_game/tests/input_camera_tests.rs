use alchimera_game::{
    build_app,
    input::{default_input_map, InputAction},
    player::{Player, PlayerCamera},
};
use bevy::prelude::{QueryState, Transform};

#[test]
fn default_input_map_contains_move_jump_interact_hotbar() {
    let input_map = default_input_map();

    assert!(input_map.contains_action(InputAction::MoveForward));
    assert!(input_map.contains_action(InputAction::MoveBackward));
    assert!(input_map.contains_action(InputAction::MoveLeft));
    assert!(input_map.contains_action(InputAction::MoveRight));
    assert!(input_map.contains_action(InputAction::Jump));
    assert!(input_map.contains_action(InputAction::Interact));
    for slot in 0..9 {
        assert!(input_map.contains_action(InputAction::HotbarSlot(slot)));
    }
}

#[test]
fn spawn_player_camera_creates_camera_companion() {
    let mut app = build_app();
    app.update();

    let mut player_query: QueryState<(bevy::prelude::Entity, &Player)> = app.world_mut().query();
    let players: Vec<_> = player_query.iter(app.world()).collect();
    assert_eq!(players.len(), 1);
    let player_entity = players[0].0;

    let mut camera_query: QueryState<(&PlayerCamera, &Transform)> = app.world_mut().query();
    let cameras: Vec<_> = camera_query.iter(app.world()).collect();

    assert_eq!(cameras.len(), 1);
    let (camera, transform) = cameras[0];
    assert_eq!(camera.player, player_entity);
    assert_eq!(transform.translation, PlayerCamera::DEFAULT_OFFSET);
}
