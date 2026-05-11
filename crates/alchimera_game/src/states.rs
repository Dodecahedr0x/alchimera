//! Game state definitions and plugin registration.

use bevy::{
    prelude::{App, AppExtStates, Plugin, States},
    state::app::StatesPlugin,
};

/// High-level runtime state for the Alchimera app shell.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// Initial state used while game resources are being prepared.
    #[default]
    Boot,
    /// Main interactive world simulation state.
    InGame,
    /// Paused simulation/menu state.
    Paused,
}

/// Registers the root game states used by runtime systems.
#[derive(Debug, Default)]
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatesPlugin).init_state::<GameState>();
    }
}
