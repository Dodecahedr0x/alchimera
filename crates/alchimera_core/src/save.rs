//! Versioned save-data shell for future persistence integration.

use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// Current supported top-level save schema version.
pub const CURRENT_SAVE_VERSION: u32 = 1;

/// Minimal top-level save container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveData {
    version: u32,
    world_seed_label: String,
}

impl SaveData {
    #[must_use]
    pub fn new(world_seed_label: impl Into<String>) -> Self {
        Self {
            version: CURRENT_SAVE_VERSION,
            world_seed_label: world_seed_label.into(),
        }
    }

    #[doc(hidden)]
    #[must_use]
    pub fn with_version_for_testing(version: u32, world_seed_label: impl Into<String>) -> Self {
        Self {
            version,
            world_seed_label: world_seed_label.into(),
        }
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub fn world_seed_label(&self) -> &str {
        &self.world_seed_label
    }

    pub const fn validate_version(&self) -> Result<(), SaveError> {
        if self.version != CURRENT_SAVE_VERSION {
            return Err(SaveError::UnsupportedVersion {
                found: self.version,
                supported: CURRENT_SAVE_VERSION,
            });
        }
        Ok(())
    }
}

/// Save load/validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
    UnsupportedVersion { found: u32, supported: u32 },
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion { found, supported } => write!(
                f,
                "unsupported save version {found}; supported version is {supported}"
            ),
        }
    }
}

impl Error for SaveError {}
