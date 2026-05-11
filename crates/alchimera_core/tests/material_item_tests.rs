use alchimera_core::{
    item::{ItemDefinition, MaterialClass},
    material::{AlchemyTrait, MaterialDefinition, MaterialProperties},
};

#[test]
fn material_definition_rejects_negative_hardness() {
    let properties = MaterialProperties {
        hardness: -1.0,
        density: 0.5,
        flammability: 0.1,
    };

    assert!(MaterialDefinition::new("stone.granite", "Granite", properties, []).is_err());
}

#[test]
fn material_can_store_alchemy_traits() {
    let properties = MaterialProperties {
        hardness: 2.0,
        density: 0.8,
        flammability: 0.0,
    };

    let material = MaterialDefinition::new(
        "wood.oak",
        "Oak Wood",
        properties,
        [AlchemyTrait::Growth, AlchemyTrait::Heat],
    )
    .expect("valid material");

    assert_eq!(
        material.traits(),
        &[AlchemyTrait::Growth, AlchemyTrait::Heat]
    );
}

#[test]
fn item_definition_references_material_class() {
    let item = ItemDefinition::new("item.log", "Log", MaterialClass::Wood, 64).expect("valid item");

    assert_eq!(item.material_class(), MaterialClass::Wood);
    assert_eq!(item.stack_limit(), 64);
}
