use alchimera_core::{
    alchemy::{
        AlchemyExperiment, AlchemyExperimentRule, AlchemyExperimentStatus, AlchemyKnowledge,
    },
    ids::ItemId,
    inventory::ItemStack,
    material::{AlchemyTrait, MaterialDefinition, MaterialProperties},
};

fn material(id: &str, traits: impl IntoIterator<Item = AlchemyTrait>) -> MaterialDefinition {
    MaterialDefinition::new(
        id,
        id,
        MaterialProperties {
            hardness: 1.0,
            density: 1.0,
            flammability: 0.0,
        },
        traits,
    )
    .expect("valid material")
}

fn item_id(value: &str) -> ItemId {
    ItemId::new(value).expect("valid item id")
}

#[test]
fn inspecting_material_discovers_trait() {
    let moss = material("moss", [AlchemyTrait::Growth, AlchemyTrait::Cold]);
    let mut knowledge = AlchemyKnowledge::default();

    let discovery = knowledge.inspect_material(&moss);

    assert_eq!(discovery.material_id(), moss.id());
    assert_eq!(
        discovery.discovered_traits(),
        &[AlchemyTrait::Growth, AlchemyTrait::Cold]
    );
    assert!(knowledge.has_discovered_trait(moss.id(), AlchemyTrait::Growth));
    assert!(knowledge.has_discovered_trait(moss.id(), AlchemyTrait::Cold));
}

#[test]
fn combining_compatible_traits_produces_reagent() {
    let ember = material("embercap", [AlchemyTrait::Heat]);
    let crystal = material("sky_crystal", [AlchemyTrait::Conductive]);
    let reagent = item_id("charged_ember_dust");
    let experiment = AlchemyExperiment::new(vec![AlchemyExperimentRule::new(
        AlchemyTrait::Heat,
        AlchemyTrait::Conductive,
        ItemStack::new(reagent.clone(), 2),
    )]);

    let result = experiment.combine(&ember, &crystal);

    assert_eq!(result.status(), AlchemyExperimentStatus::Success);
    assert_eq!(result.output(), Some(&ItemStack::new(reagent, 2)));
    assert_eq!(
        result.matched_traits(),
        Some((AlchemyTrait::Heat, AlchemyTrait::Conductive))
    );
}

#[test]
fn unknown_combination_returns_failed_experiment_result() {
    let stone = material("river_stone", [AlchemyTrait::Stability]);
    let frost = material("frost_leaf", [AlchemyTrait::Cold]);
    let experiment = AlchemyExperiment::new(vec![AlchemyExperimentRule::new(
        AlchemyTrait::Heat,
        AlchemyTrait::Conductive,
        ItemStack::new(item_id("charged_ember_dust"), 1),
    )]);

    let result = experiment.combine(&stone, &frost);

    assert_eq!(result.status(), AlchemyExperimentStatus::Failed);
    assert_eq!(result.output(), None);
    assert_eq!(result.matched_traits(), None);
}
