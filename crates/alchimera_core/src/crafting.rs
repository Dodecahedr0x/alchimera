//! Pure recipe matching and craft result planning.

use std::{error::Error, fmt};

use crate::{
    ids::{IdError, ItemId, RecipeId},
    inventory::ItemStack,
    item::MaterialClass,
};

/// A pure crafting recipe with inputs and one item output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    id: RecipeId,
    inputs: Vec<RecipeInput>,
    output: ItemStack,
}

impl Recipe {
    pub fn new(
        id: impl Into<String>,
        inputs: Vec<RecipeInput>,
        output_item: ItemId,
        output_quantity: u16,
    ) -> Result<Self, CraftingError> {
        if output_quantity == 0 {
            return Err(CraftingError::InvalidQuantity);
        }

        Ok(Self {
            id: RecipeId::new(id).map_err(CraftingError::InvalidRecipeId)?,
            inputs,
            output: ItemStack::new(output_item, output_quantity),
        })
    }

    #[must_use]
    pub const fn id(&self) -> &RecipeId {
        &self.id
    }

    #[must_use]
    pub fn inputs(&self) -> &[RecipeInput] {
        &self.inputs
    }

    #[must_use]
    pub const fn output(&self) -> &ItemStack {
        &self.output
    }

    pub fn match_inputs(
        &self,
        available: &[AvailableIngredient],
    ) -> Result<CraftPlan, CraftingError> {
        let mut remaining: Vec<u16> = available
            .iter()
            .map(AvailableIngredient::quantity)
            .collect();
        let mut consumed = Vec::new();

        for (input_index, input) in self.inputs.iter().enumerate() {
            let mut needed = input.quantity();
            let mut available_for_input = 0;

            for (ingredient_index, ingredient) in available.iter().enumerate() {
                if !input.matches(ingredient) {
                    continue;
                }

                available_for_input += remaining[ingredient_index];
                let taken = needed.min(remaining[ingredient_index]);
                if taken == 0 {
                    continue;
                }

                remaining[ingredient_index] -= taken;
                needed -= taken;
                consumed.push(CraftInputConsumption::new(
                    ingredient.item_id.clone(),
                    taken,
                ));

                if needed == 0 {
                    break;
                }
            }

            if needed != 0 {
                return Err(CraftingError::MissingInput {
                    input_index,
                    required: input.quantity(),
                    available: available_for_input,
                });
            }
        }

        Ok(CraftPlan {
            consumed,
            output: self.output.clone(),
        })
    }
}

/// One required recipe input, either exact item ID or material class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeInput {
    kind: RecipeInputKind,
    quantity: u16,
}

impl RecipeInput {
    #[must_use]
    pub const fn exact_item(item_id: ItemId, quantity: u16) -> Self {
        Self {
            kind: RecipeInputKind::ExactItem(item_id),
            quantity,
        }
    }

    #[must_use]
    pub const fn material_class(material_class: MaterialClass, quantity: u16) -> Self {
        Self {
            kind: RecipeInputKind::MaterialClass(material_class),
            quantity,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &RecipeInputKind {
        &self.kind
    }

    #[must_use]
    pub const fn quantity(&self) -> u16 {
        self.quantity
    }

    fn matches(&self, ingredient: &AvailableIngredient) -> bool {
        match &self.kind {
            RecipeInputKind::ExactItem(item_id) => item_id == &ingredient.item_id,
            RecipeInputKind::MaterialClass(material_class) => {
                material_class == &ingredient.material_class
            }
        }
    }
}

/// Selector for a required recipe input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeInputKind {
    ExactItem(ItemId),
    MaterialClass(MaterialClass),
}

/// Available item quantity with its material class for matching flexible recipes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableIngredient {
    item_id: ItemId,
    material_class: MaterialClass,
    quantity: u16,
}

impl AvailableIngredient {
    #[must_use]
    pub const fn new(item_id: ItemId, material_class: MaterialClass, quantity: u16) -> Self {
        Self {
            item_id,
            material_class,
            quantity,
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
    pub const fn quantity(&self) -> u16 {
        self.quantity
    }
}

/// Pure craft result plan: consume these inputs, then grant this output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraftPlan {
    consumed: Vec<CraftInputConsumption>,
    output: ItemStack,
}

impl CraftPlan {
    #[must_use]
    pub fn consumed(&self) -> &[CraftInputConsumption] {
        &self.consumed
    }

    #[must_use]
    pub const fn output(&self) -> &ItemStack {
        &self.output
    }
}

/// Quantity to remove from a specific item stack as part of a craft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraftInputConsumption {
    item_id: ItemId,
    quantity: u16,
}

impl CraftInputConsumption {
    #[must_use]
    pub const fn new(item_id: ItemId, quantity: u16) -> Self {
        Self { item_id, quantity }
    }

    #[must_use]
    pub const fn item_id(&self) -> &ItemId {
        &self.item_id
    }

    #[must_use]
    pub const fn quantity(&self) -> u16 {
        self.quantity
    }
}

/// Crafting operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CraftingError {
    InvalidRecipeId(IdError),
    InvalidQuantity,
    MissingInput {
        input_index: usize,
        required: u16,
        available: u16,
    },
}

impl fmt::Display for CraftingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecipeId(error) => write!(f, "invalid recipe id: {error}"),
            Self::InvalidQuantity => f.write_str("recipe quantities must be greater than zero"),
            Self::MissingInput {
                input_index,
                required,
                available,
            } => write!(
                f,
                "missing recipe input {input_index}: required {required}, available {available}"
            ),
        }
    }
}

impl Error for CraftingError {}
