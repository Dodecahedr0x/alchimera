use alchimera_generation::objects::{
    load_object_prototype_ron, ObjectPrototypeDefinitionError, ObjectPrototypeGenerator,
};

#[test]
fn prototype_rejects_missing_generator() {
    let ron = r#"
        (
            id: "objects/test_without_generator",
            display_name: "Broken Prototype",
            material_refs: ["materials/oak_wood"],
        )
    "#;

    let error = load_object_prototype_ron(ron).expect_err("prototype without generator must fail");

    assert!(matches!(
        error,
        ObjectPrototypeDefinitionError::MissingGenerator
    ));
}

#[test]
fn prototype_rejects_missing_material_reference() {
    let ron = r#"
        (
            id: "objects/test_without_materials",
            display_name: "Broken Prototype",
            generator: Tree,
            material_refs: [],
        )
    "#;

    let error = load_object_prototype_ron(ron).expect_err("prototype without materials must fail");

    assert!(matches!(
        error,
        ObjectPrototypeDefinitionError::MissingMaterialReference
    ));
}

#[test]
fn oak_tree_example_loads_and_validates() {
    let prototype = load_object_prototype_ron(include_str!(
        "../../../assets/data/objects/examples/oak_tree.ron"
    ))
    .expect("oak tree prototype should load");

    assert_eq!(prototype.id().as_str(), "objects/oak_tree");
    assert_eq!(prototype.display_name(), "Oak Tree");
    assert_eq!(prototype.generator(), ObjectPrototypeGenerator::Tree);
    assert_eq!(prototype.material_refs()[0].as_str(), "materials/oak_wood");
}

#[test]
fn granite_boulder_example_loads_and_validates() {
    let prototype = load_object_prototype_ron(include_str!(
        "../../../assets/data/objects/examples/granite_boulder.ron"
    ))
    .expect("granite boulder prototype should load");

    assert_eq!(prototype.id().as_str(), "objects/granite_boulder");
    assert_eq!(prototype.display_name(), "Granite Boulder");
    assert_eq!(prototype.generator(), ObjectPrototypeGenerator::Rock);
    assert_eq!(prototype.material_refs()[0].as_str(), "materials/granite");
}
