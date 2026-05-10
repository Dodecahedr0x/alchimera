//! Seed domain types and deterministic seed derivation.

/// Root seed for deterministic world generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldSeed(u64);

impl WorldSeed {
    /// Creates a world seed from a raw `u64`.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Returns the raw seed value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Derives a deterministic child seed from a label, signed coordinate path, and local index.
    ///
    /// This intentionally uses an explicit FNV-1a style mixer instead of Rust's default hasher,
    /// because procedural generation must remain stable across processes and platforms.
    #[must_use]
    pub fn derive_child(self, label: &str, coordinates: &[i32], index: u64) -> Self {
        let mut hash = FNV_OFFSET_BASIS;
        hash = mix_u64(hash, self.0);
        hash = mix_bytes(hash, label.as_bytes());
        hash = mix_u64(hash, coordinates.len() as u64);
        for coordinate in coordinates {
            hash = mix_u64(hash, *coordinate as i64 as u64);
        }
        hash = mix_u64(hash, index);
        Self(hash)
    }
}

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn mix_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn mix_u64(hash: u64, value: u64) -> u64 {
    mix_bytes(hash, &value.to_le_bytes())
}
