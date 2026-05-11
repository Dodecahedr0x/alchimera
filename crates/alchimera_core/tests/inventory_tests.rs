use alchimera_core::{
    ids::ItemId,
    inventory::{Inventory, InventoryError},
};

fn item_id(value: &str) -> ItemId {
    ItemId::new(value).expect("valid item id")
}

#[test]
fn inventory_adding_same_item_merges_until_stack_limit() {
    let item = item_id("item.log");
    let mut inventory = Inventory::with_slot_count(2);

    assert_eq!(inventory.add_items(item.clone(), 20, 64).unwrap(), 0);
    assert_eq!(inventory.add_items(item.clone(), 40, 64).unwrap(), 0);

    assert_eq!(inventory.slots()[0].as_ref().unwrap().quantity(), 60);
    assert_eq!(inventory.occupied_slot_count(), 1);
}

#[test]
fn inventory_adding_overflow_creates_new_stack() {
    let item = item_id("item.log");
    let mut inventory = Inventory::with_slot_count(2);

    assert_eq!(inventory.add_items(item, 70, 64).unwrap(), 0);

    assert_eq!(inventory.slots()[0].as_ref().unwrap().quantity(), 64);
    assert_eq!(inventory.slots()[1].as_ref().unwrap().quantity(), 6);
}

#[test]
fn inventory_removing_items_decrements_stack() {
    let item = item_id("item.log");
    let mut inventory = Inventory::with_slot_count(2);
    inventory.add_items(item.clone(), 70, 64).unwrap();

    inventory.remove_items(&item, 12).unwrap();

    assert_eq!(inventory.total_quantity(&item), 58);
}

#[test]
fn inventory_removing_too_many_returns_error() {
    let item = item_id("item.log");
    let mut inventory = Inventory::with_slot_count(1);
    inventory.add_items(item.clone(), 5, 64).unwrap();

    assert_eq!(
        inventory.remove_items(&item, 6),
        Err(InventoryError::InsufficientQuantity {
            available: 5,
            requested: 6
        })
    );
}
