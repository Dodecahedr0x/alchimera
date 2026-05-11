//! Material definitions and alchemical traits.

use std::{error::Error, fmt};

use crate::ids::{IdError, MaterialId};

/// Data-driven material definition used by generation, crafting, and alchemy.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialDefinition {
    id: MaterialId,
    display_name: String,
    properties: MaterialProperties,
    traits: Vec<AlchemyTrait>,
}

impl MaterialDefinition {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
        properties: MaterialProperties,
        traits: impl IntoIterator<Item = AlchemyTrait>,
    ) -> Result<Self, MaterialDefinitionError> {
        properties.validate()?;
        Ok(Self {
            id: MaterialId::new(id).map_err(MaterialDefinitionError::InvalidId)?,
            display_name: display_name.into(),
            properties,
            traits: traits.into_iter().collect(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> &MaterialId {
        &self.id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub const fn properties(&self) -> &MaterialProperties {
        &self.properties
    }

    #[must_use]
    pub fn traits(&self) -> &[AlchemyTrait] {
        &self.traits
    }
}

/// Numeric material properties used by pure gameplay rules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialProperties {
    pub hardness: f32,
    pub density: f32,
    pub flammability: f32,
}

impl MaterialProperties {
    fn validate(self) -> Result<(), MaterialDefinitionError> {
        validate_non_negative("hardness", self.hardness)?;
        validate_non_negative("density", self.density)?;
        if !(0.0..=1.0).contains(&self.flammability) {
            return Err(MaterialDefinitionError::NumericOutOfRange {
                field: "flammability",
            });
        }
        Ok(())
    }
}

/// Initial alchemical trait tags for materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlchemyTrait {
    Growth,
    Heat,
    Cold,
    Stability,
    Volatility,
    Conductive,
}

/// Validation failures for material definitions.
#[derive(Debug, Clone, PartialEq)]
pub enum MaterialDefinitionError {
    InvalidId(IdError),
    NumericOutOfRange { field: &'static str },
}

impl fmt::Display for MaterialDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(error) => write!(f, "invalid material id: {error}"),
            Self::NumericOutOfRange { field } => {
                write!(f, "material field {field} is out of range")
            }
        }
    }
}

impl Error for MaterialDefinitionError {}

fn validate_non_negative(field: &'static str, value: f32) -> Result<(), MaterialDefinitionError> {
    if value < 0.0 || value.is_nan() {
        return Err(MaterialDefinitionError::NumericOutOfRange { field });
    }
    Ok(())
}
