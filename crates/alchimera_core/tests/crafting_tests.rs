use alchimera_core::{
    crafting::{AvailableIngredient, CraftingError, Recipe, RecipeInput},
    ids::ItemId,
    item::MaterialClass,
};

fn item_id(value: &str) -> ItemId {
    ItemId::new(value).expect("valid item id")
}

fn ingredient(item: &str, class: MaterialClass, quantity: u16) -> AvailableIngredient {
    AvailableIngredient::new(item_id(item), class, quantity)
}

#[test]
fn recipe_matches_exact_item_inputs() {
    let recipe = Recipe::new(
        "tool.torch",
        vec![RecipeInput::exact_item(item_id("item.stick"), 2)],
        item_id("item.torch"),
        4,
    )
    .expect("valid recipe");
    let available = [ingredient("item.stick", MaterialClass::Wood, 3)];

    let plan = recipe
        .match_inputs(&available)
        .expect("recipe should match");

    assert_eq!(plan.output().item_id(), &item_id("item.torch"));
    assert_eq!(plan.output().quantity(), 4);
    assert_eq!(plan.consumed()[0].item_id(), &item_id("item.stick"));
    assert_eq!(plan.consumed()[0].quantity(), 2);
}

#[test]
fn recipe_matches_material_class_inputs() {
    let recipe = Recipe::new(
        "tool.stone_axe",
        vec![
            RecipeInput::material_class(MaterialClass::Stone, 1),
            RecipeInput::material_class(MaterialClass::Wood, 1),
            RecipeInput::material_class(MaterialClass::Fiber, 1),
        ],
        item_id("item.stone_axe"),
        1,
    )
    .expect("valid recipe");
    let available = [
        ingredient("item.flint", MaterialClass::Stone, 1),
        ingredient("item.branch", MaterialClass::Wood, 2),
        ingredient("item.vine", MaterialClass::Fiber, 3),
    ];

    let plan = recipe
        .match_inputs(&available)
        .expect("recipe should match");

    assert_eq!(plan.consumed().len(), 3);
    assert_eq!(plan.consumed()[0].item_id(), &item_id("item.flint"));
    assert_eq!(plan.output().item_id(), &item_id("item.stone_axe"));
}

#[test]
fn recipe_rejects_missing_required_input() {
    let recipe = Recipe::new(
        "tool.stone_axe",
        vec![
            RecipeInput::material_class(MaterialClass::Stone, 2),
            RecipeInput::material_class(MaterialClass::Wood, 1),
        ],
        item_id("item.stone_axe"),
        1,
    )
    .expect("valid recipe");
    let available = [ingredient("item.flint", MaterialClass::Stone, 1)];

    assert_eq!(
        recipe.match_inputs(&available),
        Err(CraftingError::MissingInput {
            input_index: 0,
            required: 2,
            available: 1,
        })
    );
}

#[test]
fn crafting_consumes_inputs_and_returns_output_plan() {
    let recipe = Recipe::new(
        "tool.stone_axe",
        vec![
            RecipeInput::material_class(MaterialClass::Stone, 1),
            RecipeInput::exact_item(item_id("item.handle"), 1),
        ],
        item_id("item.stone_axe"),
        1,
    )
    .expect("valid recipe");
    let available = [
        ingredient("item.flint", MaterialClass::Stone, 3),
        ingredient("item.handle", MaterialClass::Wood, 1),
    ];

    let plan = recipe
        .match_inputs(&available)
        .expect("recipe should match");

    assert_eq!(plan.consumed().len(), 2);
    assert_eq!(plan.consumed()[0].item_id(), &item_id("item.flint"));
    assert_eq!(plan.consumed()[0].quantity(), 1);
    assert_eq!(plan.consumed()[1].item_id(), &item_id("item.handle"));
    assert_eq!(plan.consumed()[1].quantity(), 1);
    assert_eq!(plan.output().item_id(), &item_id("item.stone_axe"));
    assert_eq!(plan.output().quantity(), 1);
}
