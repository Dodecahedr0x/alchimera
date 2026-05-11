use alchimera_game::{
    build_app,
    player::{MovementIntent, Player, PlayerMovementInput, PlayerStats},
};
use bevy::prelude::{QueryState, Transform, Vec3, With};

#[test]
fn spawn_player_adds_required_components() {
    let mut app = build_app();
    app.update();

    let mut query: QueryState<(&Player, &PlayerStats, &MovementIntent, &Transform)> =
        app.world_mut().query();
    let players: Vec<_> = query.iter(app.world()).collect();

    assert_eq!(players.len(), 1);
    let (_player, stats, intent, transform) = players[0];
    assert_eq!(stats.walk_speed, 5.0);
    assert_eq!(stats.jump_strength, 6.0);
    assert_eq!(intent.direction, Vec3::ZERO);
    assert!(!intent.wants_jump);
    assert_eq!(transform.translation, Vec3::ZERO);
}

#[test]
fn movement_input_updates_intent_component() {
    let mut app = build_app();
    app.update();

    app.world_mut().send_event(PlayerMovementInput {
        direction: Vec3::new(1.0, 0.0, -1.0),
        wants_jump: true,
    });
    app.update();

    let mut query: QueryState<&MovementIntent, With<Player>> = app.world_mut().query_filtered();
    let intent = query.single(app.world());

    assert_eq!(intent.direction, Vec3::new(1.0, 0.0, -1.0).normalize());
    assert!(intent.wants_jump);
}
