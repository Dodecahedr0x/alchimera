use alchimera_core::{
    error::ValidationError,
    validation::{ensure_f32_in_range, ensure_reference_exists, ensure_unique_ids},
};

#[test]
fn duplicate_ids_are_rejected() {
    let ids = ["item.stone", "item.branch", "item.stone"];

    assert_eq!(
        ensure_unique_ids(ids, "item definitions"),
        Err(ValidationError::DuplicateId {
            id: "item.stone".to_string(),
            context: "item definitions".to_string(),
        })
    );
}

#[test]
fn missing_reference_is_reported_with_context() {
    let known_materials = ["material.oak", "material.flint"];

    assert_eq!(
        ensure_reference_exists("material.copper", known_materials, "item.pickaxe.material"),
        Err(ValidationError::MissingReference {
            id: "material.copper".to_string(),
            context: "item.pickaxe.material".to_string(),
        })
    );
}

#[test]
fn numeric_range_validation_reports_field_name() {
    assert_eq!(
        ensure_f32_in_range("material.flammability", 1.5, 0.0, 1.0),
        Err(ValidationError::NumericOutOfRange {
            field: "material.flammability".to_string(),
            value: 1.5,
            min: 0.0,
            max: 1.0,
        })
    );
}
