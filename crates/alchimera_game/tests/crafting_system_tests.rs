use alchimera_core::{
    crafting::{Recipe, RecipeInput},
    ids::ItemId,
    item::MaterialClass,
};
use alchimera_game::{
    crafting::{
        CraftRecipe, HandCraftingFailures, HandCraftingPlugin, ItemCraftingDefinition,
        ItemCraftingDefinitions,
    },
    harvesting::PlayerInventory,
};
use bevy::prelude::{App, MinimalPlugins};

#[test]
fn craft_event_consumes_inventory_inputs() {
    let wood = item_id("alchimera:wood");
    let stone_axe = item_id("alchimera:stone_axe");
    let recipe = Recipe::new(
        "alchimera:stone_axe",
        vec![RecipeInput::exact_item(wood.clone(), 2)],
        stone_axe.clone(),
        1,
    )
    .unwrap();
    let mut app = crafting_app();
    register_item(&mut app, wood.clone(), MaterialClass::Wood, 64);
    register_item(&mut app, stone_axe, MaterialClass::Stone, 1);
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(wood.clone(), 3, 64);

    app.world_mut().send_event(CraftRecipe::new(recipe));
    app.update();

    assert_eq!(
        app.world()
            .resource::<PlayerInventory>()
            .total_quantity(&wood),
        1
    );
}

#[test]
fn craft_event_adds_output_item() {
    let wood = item_id("alchimera:wood");
    let stone_axe = item_id("alchimera:stone_axe");
    let recipe = Recipe::new(
        "alchimera:stone_axe",
        vec![RecipeInput::material_class(MaterialClass::Wood, 2)],
        stone_axe.clone(),
        1,
    )
    .unwrap();
    let mut app = crafting_app();
    register_item(&mut app, wood.clone(), MaterialClass::Wood, 64);
    register_item(&mut app, stone_axe.clone(), MaterialClass::Stone, 1);
    app.world_mut()
        .resource_mut::<PlayerInventory>()
        .add_items(wood, 2, 64);

    app.world_mut().send_event(CraftRecipe::new(recipe));
    app.update();

    assert_eq!(
        app.world()
            .resource::<PlayerInventory>()
            .total_quantity(&stone_axe),
        1
    );
}

#[test]
fn craft_event_fails_without_required_inputs() {
    let wood = item_id("alchimera:wood");
    let stone_axe = item_id("alchimera:stone_axe");
    let recipe = Recipe::new(
        "alchimera:stone_axe",
        vec![RecipeInput::exact_item(wood, 2)],
        stone_axe,
        1,
    )
    .unwrap();
    let mut app = crafting_app();

    app.world_mut().send_event(CraftRecipe::new(recipe));
    app.update();

    assert_eq!(app.world().resource::<HandCraftingFailures>().len(), 1);
}

fn crafting_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, HandCraftingPlugin));
    app
}

fn register_item(app: &mut App, item_id: ItemId, material_class: MaterialClass, stack_limit: u16) {
    app.world_mut()
        .resource_mut::<ItemCraftingDefinitions>()
        .insert(ItemCraftingDefinition::new(
            item_id,
            material_class,
            stack_limit,
        ));
}

fn item_id(value: &str) -> ItemId {
    ItemId::new(value).unwrap()
}
