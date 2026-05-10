//! Deterministic world-generation logic for Alchimera.

pub mod biome;
pub mod chunk;
pub mod mesh;
pub mod modification_log;
pub mod objects;
pub mod terrain;

/// Stable crate identifier used by bootstrap smoke tests.
pub const CRATE_NAME: &str = "alchimera_generation";

/// Returns the name of the core crate dependency to prove this crate is wired.
#[must_use]
pub fn core_crate_name() -> &'static str {
    alchimera_core::CRATE_NAME
}

#[cfg(test)]
mod tests {
    use super::{core_crate_name, CRATE_NAME};

    #[test]
    fn crate_is_addressable() {
        assert_eq!(CRATE_NAME, "alchimera_generation");
    }

    #[test]
    fn core_dependency_is_addressable() {
        assert_eq!(core_crate_name(), "alchimera_core");
    }

    #[test]
    fn module_smoke_generation_modules_are_addressable() {
        #[allow(unused_imports)]
        use crate::{biome, chunk, mesh, modification_log, objects, terrain};
    }
}
