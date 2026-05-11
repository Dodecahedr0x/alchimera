//! Shared validation helpers for data-driven definitions.

use std::collections::HashSet;

use crate::error::ValidationError;

pub fn ensure_unique_ids<I, S>(ids: I, context: impl Into<String>) -> Result<(), ValidationError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let context = context.into();
    let mut seen = HashSet::new();

    for id in ids {
        let id = id.as_ref();
        if !seen.insert(id.to_string()) {
            return Err(ValidationError::DuplicateId {
                id: id.to_string(),
                context,
            });
        }
    }

    Ok(())
}

pub fn ensure_reference_exists<I, S>(
    id: impl AsRef<str>,
    known_ids: I,
    context: impl Into<String>,
) -> Result<(), ValidationError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let id = id.as_ref();
    if known_ids
        .into_iter()
        .any(|known_id| known_id.as_ref() == id)
    {
        return Ok(());
    }

    Err(ValidationError::MissingReference {
        id: id.to_string(),
        context: context.into(),
    })
}

pub fn ensure_f32_in_range(
    field: impl Into<String>,
    value: f32,
    min: f32,
    max: f32,
) -> Result<(), ValidationError> {
    if value.is_nan() || value < min || value > max {
        return Err(ValidationError::NumericOutOfRange {
            field: field.into(),
            value,
            min,
            max,
        });
    }

    Ok(())
}
