//! Core domain types and pure logic for Alchimera.

/// Stable crate identifier used by bootstrap smoke tests.
pub const CRATE_NAME: &str = "alchimera_core";

#[cfg(test)]
mod tests {
    use super::CRATE_NAME;

    #[test]
    fn crate_is_addressable() {
        assert_eq!(CRATE_NAME, "alchimera_core");
    }
}
