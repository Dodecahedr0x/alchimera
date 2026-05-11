//! Inventory and hotbar UI shell resources.

use alchimera_core::inventory::Inventory;
use bevy::prelude::{App, Commands, Component, Plugin, Resource, Startup};

/// Selection behavior at the ends of the hotbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotbarSelectionMode {
    Wrap,
    Clamp,
}

/// Current hotbar selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotbarSelection {
    selected_slot: usize,
    slot_count: usize,
    mode: HotbarSelectionMode,
}

impl HotbarSelection {
    #[must_use]
    pub fn new(selected_slot: usize, slot_count: usize, mode: HotbarSelectionMode) -> Self {
        let mut selection = Self {
            selected_slot: 0,
            slot_count: slot_count.max(1),
            mode,
        };
        selection.select_slot(selected_slot);
        selection
    }

    #[must_use]
    pub const fn selected_slot(&self) -> usize {
        self.selected_slot
    }

    #[must_use]
    pub const fn slot_count(&self) -> usize {
        self.slot_count
    }

    pub fn select_next(&mut self) {
        self.select_slot(self.selected_slot + 1);
    }

    pub fn select_previous(&mut self) {
        if self.selected_slot == 0 {
            match self.mode {
                HotbarSelectionMode::Wrap => self.selected_slot = self.slot_count - 1,
                HotbarSelectionMode::Clamp => self.selected_slot = 0,
            }
        } else {
            self.selected_slot -= 1;
        }
    }

    pub fn select_slot(&mut self, slot: usize) {
        self.selected_slot = match self.mode {
            HotbarSelectionMode::Wrap => slot % self.slot_count,
            HotbarSelectionMode::Clamp => slot.min(self.slot_count - 1),
        };
    }
}

impl Default for HotbarSelection {
    fn default() -> Self {
        Self::new(0, 8, HotbarSelectionMode::Wrap)
    }
}

/// Bevy-facing inventory UI model.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct InventoryUiState {
    inventory: Inventory,
    hotbar: HotbarSelection,
}

impl InventoryUiState {
    #[must_use]
    pub fn with_slot_counts(inventory_slots: usize, hotbar_slots: usize) -> Self {
        Self {
            inventory: Inventory::with_slot_count(inventory_slots),
            hotbar: HotbarSelection::new(0, hotbar_slots, HotbarSelectionMode::Wrap),
        }
    }

    #[must_use]
    pub const fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    #[must_use]
    pub const fn hotbar(&self) -> &HotbarSelection {
        &self.hotbar
    }

    #[must_use]
    pub const fn hotbar_mut(&mut self) -> &mut HotbarSelection {
        &mut self.hotbar
    }
}

impl Default for InventoryUiState {
    fn default() -> Self {
        Self::with_slot_counts(24, 8)
    }
}

/// Marker for the hotbar UI root entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct HotbarUiShell;

/// Registers inventory/hotbar UI state and shell entity.
#[derive(Debug, Default)]
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryUiState>()
            .add_systems(Startup, spawn_hotbar_ui_shell);
    }
}

fn spawn_hotbar_ui_shell(mut commands: Commands) {
    commands.spawn(HotbarUiShell);
}
