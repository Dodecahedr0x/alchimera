//! Shared core validation error types.

use std::{error::Error, fmt};

/// Validation failures for data-driven definitions and references.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    DuplicateId {
        id: String,
        context: String,
    },
    MissingReference {
        id: String,
        context: String,
    },
    NumericOutOfRange {
        field: String,
        value: f32,
        min: f32,
        max: f32,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId { id, context } => {
                write!(f, "duplicate id {id} in {context}")
            }
            Self::MissingReference { id, context } => {
                write!(f, "missing reference {id} in {context}")
            }
            Self::NumericOutOfRange {
                field,
                value,
                min,
                max,
            } => write!(
                f,
                "numeric field {field}={value} is outside inclusive range {min}..={max}"
            ),
        }
    }
}

impl Error for ValidationError {}
