//! Strongly typed stable identifiers for Alchimera domain objects.

use std::{error::Error, fmt};

use crate::seed::WorldSeed;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                validate_string_id(stringify!($name), &value)?;
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_id!(PrototypeId);
string_id!(MaterialId);
string_id!(ItemId);
string_id!(RecipeId);
string_id!(ChunkId);

/// Stable identifier for a generated object instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(u64);

impl ObjectId {
    /// Derives an object ID from deterministic generation inputs.
    #[must_use]
    pub fn from_seed_chunk_and_index(seed: WorldSeed, chunk: [i32; 2], index: u64) -> Self {
        Self(seed.derive_child("object.instance", &chunk, index).as_u64())
    }

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object.{:016x}", self.0)
    }
}

/// Validation error for stable string identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    Empty { type_name: &'static str },
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { type_name } => write!(f, "{type_name} cannot be empty"),
        }
    }
}

impl Error for IdError {}

fn validate_string_id(type_name: &'static str, value: &str) -> Result<(), IdError> {
    if value.trim().is_empty() {
        return Err(IdError::Empty { type_name });
    }
    Ok(())
}
