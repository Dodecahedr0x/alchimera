//! Save/load integration for gameplay runtime state.

use alchimera_core::{ids::ItemId, seed::WorldSeed};
use alchimera_generation::{chunk::ChunkCoord, modification_log::ChunkModificationLog};
use bevy::prelude::{Entity, Resource, Transform, World};

use crate::{
    building::PlayerBuiltObject,
    harvesting::{ChunkModificationLogs, PlayerInventory},
    objects::WorldObject,
};

/// Runtime resource storing the deterministic world seed that should persist across save/load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub struct PersistentWorldSeed {
    seed: WorldSeed,
}

impl PersistentWorldSeed {
    #[must_use]
    pub const fn new(seed: WorldSeed) -> Self {
        Self { seed }
    }

    #[must_use]
    pub const fn seed(self) -> WorldSeed {
        self.seed
    }
}

/// Complete gameplay state snapshot for the current vertical slice.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistedGameState {
    world_seed: WorldSeed,
    inventory_slot_count: usize,
    inventory_stacks: Vec<PersistedItemStack>,
    chunk_logs: Vec<ChunkModificationLog>,
    placed_objects: Vec<PersistedPlacedObject>,
}

/// Persisted item stack entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedItemStack {
    item_id: ItemId,
    quantity: u16,
}

/// Persisted player-authored world object entry.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistedPlacedObject {
    world_object: WorldObject,
    chunk: ChunkCoord,
    transform: Transform,
}

/// Captures the save/load-relevant resources and player-authored entities from a Bevy world.
#[must_use]
pub fn save_game_state(world: &mut World) -> PersistedGameState {
    let world_seed = world
        .get_resource::<PersistentWorldSeed>()
        .copied()
        .unwrap_or_else(|| PersistentWorldSeed::new(WorldSeed::new(0)))
        .seed();
    let inventory = world.resource::<PlayerInventory>();
    let logs = world.resource::<ChunkModificationLogs>();

    PersistedGameState {
        world_seed,
        inventory_slot_count: inventory.slot_count(),
        inventory_stacks: inventory
            .item_stacks()
            .map(|stack| PersistedItemStack {
                item_id: stack.item_id().clone(),
                quantity: stack.quantity(),
            })
            .collect(),
        chunk_logs: logs.iter().cloned().collect(),
        placed_objects: collect_placed_objects(world),
    }
}

/// Replaces save/load-relevant resources and placed-object entities with a saved snapshot.
pub fn load_game_state(world: &mut World, save: PersistedGameState) {
    despawn_existing_player_built_objects(world);

    world.insert_resource(PersistentWorldSeed::new(save.world_seed));

    let mut inventory = PlayerInventory::with_slot_count(save.inventory_slot_count);
    for stack in save.inventory_stacks {
        inventory.add_items(stack.item_id, stack.quantity, stack.quantity.max(1));
    }
    world.insert_resource(inventory);

    let mut logs = ChunkModificationLogs::default();
    for log in save.chunk_logs {
        logs.insert_log(log);
    }
    world.insert_resource(logs);

    for placed in save.placed_objects {
        world.spawn((
            placed.world_object,
            PlayerBuiltObject {
                chunk: placed.chunk,
            },
            placed.transform,
        ));
    }
}

fn collect_placed_objects(world: &mut World) -> Vec<PersistedPlacedObject> {
    let mut query = world
        .query_filtered::<(&WorldObject, &PlayerBuiltObject, &Transform), WithPlayerBuiltObject>();
    query
        .iter(world)
        .map(|(world_object, built, transform)| PersistedPlacedObject {
            world_object: *world_object,
            chunk: built.chunk,
            transform: *transform,
        })
        .collect()
}

fn despawn_existing_player_built_objects(world: &mut World) {
    let mut query = world.query_filtered::<Entity, WithPlayerBuiltObject>();
    let entities: Vec<_> = query.iter(world).collect();
    for entity in entities {
        world.despawn(entity);
    }
}

type WithPlayerBuiltObject = bevy::prelude::With<PlayerBuiltObject>;
