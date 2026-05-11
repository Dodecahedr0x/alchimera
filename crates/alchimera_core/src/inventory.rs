//! Pure inventory slot and stack behavior.

use std::{error::Error, fmt};

use crate::ids::ItemId;

/// Fixed-size inventory made of optional item stacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    #[must_use]
    pub fn with_slot_count(slot_count: usize) -> Self {
        Self {
            slots: vec![None; slot_count],
        }
    }

    /// Adds items, merging compatible stacks before opening new stacks.
    ///
    /// Returns the quantity that could not be inserted because the inventory filled.
    pub fn add_items(
        &mut self,
        item_id: ItemId,
        quantity: u16,
        stack_limit: u16,
    ) -> Result<u16, InventoryError> {
        if stack_limit == 0 {
            return Err(InventoryError::InvalidStackLimit);
        }

        let mut remaining = quantity;

        for stack in self
            .slots
            .iter_mut()
            .filter_map(Option::as_mut)
            .filter(|stack| stack.item_id == item_id)
        {
            remaining = stack.add_up_to_limit(remaining, stack_limit);
            if remaining == 0 {
                return Ok(0);
            }
        }

        for slot in self.slots.iter_mut().filter(|slot| slot.is_none()) {
            let inserted = remaining.min(stack_limit);
            *slot = Some(ItemStack::new(item_id.clone(), inserted));
            remaining -= inserted;
            if remaining == 0 {
                return Ok(0);
            }
        }

        Ok(remaining)
    }

    pub fn remove_items(&mut self, item_id: &ItemId, quantity: u16) -> Result<(), InventoryError> {
        let available = self.total_quantity(item_id);
        if available < quantity {
            return Err(InventoryError::InsufficientQuantity {
                available,
                requested: quantity,
            });
        }

        let mut remaining = quantity;
        for slot in self.slots.iter_mut() {
            let Some(stack) = slot.as_mut() else {
                continue;
            };
            if &stack.item_id != item_id {
                continue;
            }

            let removed = stack.remove_up_to(remaining);
            remaining -= removed;
            if stack.quantity == 0 {
                *slot = None;
            }
            if remaining == 0 {
                return Ok(());
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn total_quantity(&self, item_id: &ItemId) -> u16 {
        self.slots
            .iter()
            .filter_map(Option::as_ref)
            .filter(|stack| &stack.item_id == item_id)
            .map(ItemStack::quantity)
            .sum()
    }

    #[must_use]
    pub fn occupied_slot_count(&self) -> usize {
        self.slots.iter().filter(|slot| slot.is_some()).count()
    }

    #[must_use]
    pub fn slots(&self) -> &[Option<ItemStack>] {
        &self.slots
    }
}

/// A stack of identical items in one inventory slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    item_id: ItemId,
    quantity: u16,
}

impl ItemStack {
    #[must_use]
    pub const fn new(item_id: ItemId, quantity: u16) -> Self {
        Self { item_id, quantity }
    }

    #[must_use]
    pub const fn item_id(&self) -> &ItemId {
        &self.item_id
    }

    #[must_use]
    pub const fn quantity(&self) -> u16 {
        self.quantity
    }

    fn add_up_to_limit(&mut self, incoming: u16, stack_limit: u16) -> u16 {
        let space = stack_limit.saturating_sub(self.quantity);
        let added = incoming.min(space);
        self.quantity += added;
        incoming - added
    }

    fn remove_up_to(&mut self, requested: u16) -> u16 {
        let removed = requested.min(self.quantity);
        self.quantity -= removed;
        removed
    }
}

/// Inventory operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    InvalidStackLimit,
    InsufficientQuantity { available: u16, requested: u16 },
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStackLimit => f.write_str("stack_limit must be greater than zero"),
            Self::InsufficientQuantity {
                available,
                requested,
            } => write!(
                f,
                "insufficient quantity: available {available}, requested {requested}"
            ),
        }
    }
}

impl Error for InventoryError {}
