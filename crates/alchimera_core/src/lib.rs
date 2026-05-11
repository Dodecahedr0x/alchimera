//! Core domain types and pure logic for Alchimera.

pub mod crafting;
pub mod error;
pub mod ids;
pub mod inventory;
pub mod item;
pub mod material;
pub mod save;
pub mod seed;

/// Stable crate identifier used by bootstrap smoke tests.
pub const CRATE_NAME: &str = "alchimera_core";

#[cfg(test)]
mod tests {
    use super::CRATE_NAME;

    #[test]
    fn crate_is_addressable() {
        assert_eq!(CRATE_NAME, "alchimera_core");
    }

    #[test]
    fn module_smoke_core_modules_are_addressable() {
        #[allow(unused_imports)]
        use crate::{crafting, error, ids, inventory, item, material, save, seed};
    }
}
