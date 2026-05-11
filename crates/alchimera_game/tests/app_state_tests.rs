use alchimera_game::{build_app, states::GameState};
use bevy::prelude::{NextState, State};

#[test]
fn app_state_registers_expected_initial_state() {
    let app = build_app();
    let state = app.world().resource::<State<GameState>>();
    let next_state = app.world().resource::<NextState<GameState>>();

    assert_eq!(*state.get(), GameState::Boot);
    assert!(matches!(next_state, NextState::Unchanged));
}

#[test]
fn app_state_can_enter_in_game_state() {
    let mut app = build_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::InGame);
    app.update();

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::InGame
    );
}
