//! Item definitions and material classes.

use std::{error::Error, fmt};

use crate::ids::{IdError, ItemId};

/// Pure data definition for an item type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDefinition {
    id: ItemId,
    display_name: String,
    material_class: MaterialClass,
    stack_limit: u16,
}

impl ItemDefinition {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
        material_class: MaterialClass,
        stack_limit: u16,
    ) -> Result<Self, ItemDefinitionError> {
        if stack_limit == 0 {
            return Err(ItemDefinitionError::InvalidStackLimit);
        }

        Ok(Self {
            id: ItemId::new(id).map_err(ItemDefinitionError::InvalidId)?,
            display_name: display_name.into(),
            material_class,
            stack_limit,
        })
    }

    #[must_use]
    pub const fn id(&self) -> &ItemId {
        &self.id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
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

/// Broad material class used by recipes that can accept equivalent materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialClass {
    Wood,
    Stone,
    Metal,
    Fiber,
    Plant,
    Soil,
    Crystal,
}

/// Validation failures for item definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemDefinitionError {
    InvalidId(IdError),
    InvalidStackLimit,
}

impl fmt::Display for ItemDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(error) => write!(f, "invalid item id: {error}"),
            Self::InvalidStackLimit => f.write_str("item stack_limit must be greater than zero"),
        }
    }
}

impl Error for ItemDefinitionError {}
