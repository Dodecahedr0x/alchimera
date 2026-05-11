use alchimera_game::inventory_ui::{
    HotbarSelection, HotbarSelectionMode, HotbarUiShell, InventoryUiPlugin, InventoryUiState,
};
use bevy::prelude::{App, MinimalPlugins};

#[test]
fn hotbar_selection_wraps_or_clamps_as_configured() {
    let mut wrapping = HotbarSelection::new(0, 5, HotbarSelectionMode::Wrap);
    wrapping.select_previous();
    assert_eq!(wrapping.selected_slot(), 4);
    wrapping.select_next();
    assert_eq!(wrapping.selected_slot(), 0);

    let mut clamping = HotbarSelection::new(0, 5, HotbarSelectionMode::Clamp);
    clamping.select_previous();
    assert_eq!(clamping.selected_slot(), 0);
    clamping.select_slot(99);
    assert_eq!(clamping.selected_slot(), 4);
}

#[test]
fn inventory_resource_initializes_with_fixed_slot_count() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InventoryUiPlugin));

    let inventory = app.world().resource::<InventoryUiState>();
    assert_eq!(inventory.inventory().slots().len(), 24);
    assert_eq!(inventory.hotbar().slot_count(), 8);
    assert_eq!(inventory.hotbar().selected_slot(), 0);
}

#[test]
fn inventory_ui_plugin_spawns_hotbar_shell_marker() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InventoryUiPlugin));
    app.update();

    let shell_count = app
        .world_mut()
        .query::<&HotbarUiShell>()
        .iter(app.world())
        .count();
    assert_eq!(shell_count, 1);
}
