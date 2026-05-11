//! Harvesting events and ECS systems.

use std::collections::HashMap;

use alchimera_core::{ids::ItemId, inventory::Inventory};
use alchimera_generation::{chunk::ChunkCoord, modification_log::ChunkModificationLog};
use bevy::prelude::{
    App, Commands, Component, Entity, Event, EventReader, Plugin, Query, ResMut, Resource, Update,
};

use crate::objects::WorldObject;

/// Player inventory resource used by gameplay ECS systems.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct PlayerInventory {
    inventory: Inventory,
}

impl PlayerInventory {
    #[must_use]
    pub fn with_slot_count(slot_count: usize) -> Self {
        Self {
            inventory: Inventory::with_slot_count(slot_count),
        }
    }

    #[must_use]
    pub fn total_quantity(&self, item_id: &ItemId) -> u16 {
        self.inventory.total_quantity(item_id)
    }

    pub fn add_items(&mut self, item_id: ItemId, quantity: u16, stack_limit: u16) -> u16 {
        self.inventory
            .add_items(item_id, quantity, stack_limit)
            .expect("harvestable stack limits must be valid")
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self::with_slot_count(24)
    }
}

/// Resource containing modification logs keyed by generated chunk.
#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct ChunkModificationLogs {
    logs: HashMap<ChunkCoord, ChunkModificationLog>,
}

impl ChunkModificationLogs {
    #[must_use]
    pub fn log_for(&self, chunk: ChunkCoord) -> Option<&ChunkModificationLog> {
        self.logs.get(&chunk)
    }

    fn log_for_mut(&mut self, chunk: ChunkCoord) -> &mut ChunkModificationLog {
        self.logs
            .entry(chunk)
            .or_insert_with(|| ChunkModificationLog::new(chunk))
    }
}

/// Component describing the inventory yield for a harvestable world object.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Harvestable {
    pub yield_item: ItemId,
    pub quantity: u16,
    pub stack_limit: u16,
    pub chunk: ChunkCoord,
}

/// Marker component inserted after a world object has been harvested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Harvested;

/// Request to harvest an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct HarvestObject {
    pub entity: Entity,
}

/// Registers harvesting resources and systems.
#[derive(Debug, Default)]
pub struct HarvestingPlugin;

impl Plugin for HarvestingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInventory>()
            .init_resource::<ChunkModificationLogs>()
            .add_event::<HarvestObject>()
            .add_systems(Update, apply_harvest_events);
    }
}

fn apply_harvest_events(
    mut commands: Commands,
    mut events: EventReader<HarvestObject>,
    mut inventory: ResMut<PlayerInventory>,
    mut logs: ResMut<ChunkModificationLogs>,
    harvestables: Query<(&WorldObject, &Harvestable, Option<&Harvested>)>,
) {
    for event in events.read() {
        let Ok((object, harvestable, harvested)) = harvestables.get(event.entity) else {
            continue;
        };
        if harvested.is_some() {
            continue;
        }

        let overflow = inventory.add_items(
            harvestable.yield_item.clone(),
            harvestable.quantity,
            harvestable.stack_limit,
        );
        if overflow == harvestable.quantity {
            continue;
        }

        commands.entity(event.entity).insert(Harvested);
        logs.log_for_mut(harvestable.chunk)
            .record_removed(object.id);
    }
}
