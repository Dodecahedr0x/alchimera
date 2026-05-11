//! Hand crafting events and ECS system.

use std::collections::HashMap;

use alchimera_core::{
    crafting::{AvailableIngredient, CraftingError, Recipe},
    ids::ItemId,
    inventory::InventoryError,
    item::MaterialClass,
};
use bevy::prelude::{App, Event, EventReader, Plugin, ResMut, Resource, Update};

use crate::harvesting::PlayerInventory;

/// Item metadata needed by the crafting ECS adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCraftingDefinition {
    item_id: ItemId,
    material_class: MaterialClass,
    stack_limit: u16,
}

impl ItemCraftingDefinition {
    #[must_use]
    pub const fn new(item_id: ItemId, material_class: MaterialClass, stack_limit: u16) -> Self {
        Self {
            item_id,
            material_class,
            stack_limit,
        }
    }

    #[must_use]
    pub const fn item_id(&self) -> &ItemId {
        &self.item_id
    }

    #[must_use]
    pub const fn material_class(&self) -> MaterialClass {
        self.material_class
    }

    #[must_use]
    pub const fn stack_limit(&self) -> u16 {
        self.stack_limit
    }
}

/// Registry mapping item IDs to the material metadata required by recipes.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct ItemCraftingDefinitions {
    definitions: HashMap<ItemId, ItemCraftingDefinition>,
}

impl ItemCraftingDefinitions {
    pub fn insert(&mut self, definition: ItemCraftingDefinition) {
        self.definitions
            .insert(definition.item_id().clone(), definition);
    }

    #[must_use]
    pub fn get(&self, item_id: &ItemId) -> Option<&ItemCraftingDefinition> {
        self.definitions.get(item_id)
    }
}

/// Event requesting that a hand recipe be applied to the player's inventory.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct CraftRecipe {
    recipe: Recipe,
}

impl CraftRecipe {
    #[must_use]
    pub const fn new(recipe: Recipe) -> Self {
        Self { recipe }
    }

    #[must_use]
    pub const fn recipe(&self) -> &Recipe {
        &self.recipe
    }
}

/// Crafting failures recorded for UI/diagnostics feedback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandCraftingFailure {
    MissingItemDefinition(ItemId),
    Crafting(CraftingError),
    Inventory(InventoryError),
    InventoryFull { item_id: ItemId, overflow: u16 },
}

/// Recent hand crafting failures.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct HandCraftingFailures {
    failures: Vec<HandCraftingFailure>,
}

impl HandCraftingFailures {
    pub fn push(&mut self, failure: HandCraftingFailure) {
        self.failures.push(failure);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.failures.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.failures.is_empty()
    }

    #[must_use]
    pub fn failures(&self) -> &[HandCraftingFailure] {
        &self.failures
    }
}

/// Registers hand crafting resources, event, and system.
#[derive(Debug, Default)]
pub struct HandCraftingPlugin;

impl Plugin for HandCraftingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCraftingDefinitions>()
            .init_resource::<HandCraftingFailures>()
            .init_resource::<PlayerInventory>()
            .add_event::<CraftRecipe>()
            .add_systems(Update, apply_craft_recipe_events);
    }
}

fn apply_craft_recipe_events(
    mut events: EventReader<CraftRecipe>,
    definitions: ResMut<ItemCraftingDefinitions>,
    mut inventory: ResMut<PlayerInventory>,
    mut failures: ResMut<HandCraftingFailures>,
) {
    for event in events.read() {
        let Some(available) = available_ingredients(&inventory, &definitions, &mut failures) else {
            continue;
        };
        let plan = match event.recipe().match_inputs(&available) {
            Ok(plan) => plan,
            Err(error) => {
                failures.push(HandCraftingFailure::Crafting(error));
                continue;
            }
        };

        let Some(output_definition) = definitions.get(plan.output().item_id()) else {
            failures.push(HandCraftingFailure::MissingItemDefinition(
                plan.output().item_id().clone(),
            ));
            continue;
        };
        let output_stack_limit = output_definition.stack_limit();

        for consumed in plan.consumed() {
            if let Err(error) = inventory.remove_items(consumed.item_id(), consumed.quantity()) {
                failures.push(HandCraftingFailure::Inventory(error));
                continue;
            }
        }

        let overflow = inventory.add_items(
            plan.output().item_id().clone(),
            plan.output().quantity(),
            output_stack_limit,
        );
        if overflow > 0 {
            failures.push(HandCraftingFailure::InventoryFull {
                item_id: plan.output().item_id().clone(),
                overflow,
            });
        }
    }
}

fn available_ingredients(
    inventory: &PlayerInventory,
    definitions: &ItemCraftingDefinitions,
    failures: &mut HandCraftingFailures,
) -> Option<Vec<AvailableIngredient>> {
    let mut available = Vec::new();
    for stack in inventory.item_stacks() {
        let Some(definition) = definitions.get(stack.item_id()) else {
            failures.push(HandCraftingFailure::MissingItemDefinition(
                stack.item_id().clone(),
            ));
            return None;
        };
        available.push(AvailableIngredient::new(
            stack.item_id().clone(),
            definition.material_class(),
            stack.quantity(),
        ));
    }
    Some(available)
}
