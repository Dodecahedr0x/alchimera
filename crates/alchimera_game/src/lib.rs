//! Bevy runtime shell for Alchimera.

use bevy::prelude::{App, MinimalPlugins};

use crate::{
    diagnostics::DiagnosticsPlugin, harvesting::HarvestingPlugin, interaction::InteractionPlugin,
    inventory_ui::InventoryUiPlugin, player::PlayerPlugin, states::GameStatePlugin,
    streaming::ChunkStreamingPlugin, terrain_rendering::TerrainRenderingPlugin,
};

pub mod diagnostics;
pub mod harvesting;
pub mod input;
pub mod interaction;
pub mod inventory_ui;
pub mod objects;
pub mod player;
pub mod states;
pub mod streaming;
pub mod terrain_rendering;
pub mod ui;
pub mod world;

/// Stable crate identifier used by bootstrap smoke tests.
pub const CRATE_NAME: &str = "alchimera_game";

/// Builds the minimal Bevy app shell used by the root binary.
#[must_use]
pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        GameStatePlugin,
        DiagnosticsPlugin,
        InteractionPlugin,
        HarvestingPlugin,
        InventoryUiPlugin,
        PlayerPlugin,
        TerrainRenderingPlugin,
        ChunkStreamingPlugin,
    ));
    app
}

/// Starts the minimal Bevy app shell.
pub fn run() {
    build_app().run();
}

/// Returns dependency crate names to prove workspace wiring.
#[must_use]
pub fn dependency_crate_names() -> (&'static str, &'static str) {
    (alchimera_core::CRATE_NAME, alchimera_generation::CRATE_NAME)
}

#[cfg(test)]
mod tests {
    use super::{build_app, dependency_crate_names, CRATE_NAME};

    #[test]
    fn crate_is_addressable() {
        assert_eq!(CRATE_NAME, "alchimera_game");
    }

    #[test]
    fn dependencies_are_addressable() {
        assert_eq!(
            dependency_crate_names(),
            ("alchimera_core", "alchimera_generation")
        );
    }

    #[test]
    fn bevy_app_shell_can_be_built() {
        let _app = build_app();
    }

    #[test]
    fn module_smoke_game_modules_are_addressable() {
        #[allow(unused_imports)]
        use crate::{
            diagnostics, harvesting, input, interaction, inventory_ui, objects, player, states,
            streaming, terrain_rendering, ui, world,
        };
    }
}
