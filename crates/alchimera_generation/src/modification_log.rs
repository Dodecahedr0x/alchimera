//! Serializable generated-chunk modification overlays.

use alchimera_core::ids::ObjectId;
use serde::{Deserialize, Serialize};

use crate::chunk::ChunkCoord;

/// Player-authored object placement that should be layered over procedural output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectPlacementOverride {
    stable_id: String,
    prototype_key: String,
    world_position: [f32; 3],
}

impl ObjectPlacementOverride {
    #[must_use]
    pub fn new(
        stable_id: impl Into<String>,
        prototype_key: impl Into<String>,
        world_position: [f32; 3],
    ) -> Self {
        Self {
            stable_id: stable_id.into(),
            prototype_key: prototype_key.into(),
            world_position,
        }
    }

    #[must_use]
    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }

    #[must_use]
    pub fn prototype_key(&self) -> &str {
        &self.prototype_key
    }

    #[must_use]
    pub const fn world_position(&self) -> [f32; 3] {
        self.world_position
    }
}

/// Generated object IDs that remain visible plus player-authored placements.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedChunkObjects {
    pub generated_object_ids: Vec<ObjectId>,
    pub placed_objects: Vec<ObjectPlacementOverride>,
}

/// Serializable override log for generated object removals, damage, and placements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkModificationLog {
    chunk: SerializableChunkCoord,
    removed_object_ids: Vec<u64>,
    damaged_object_ids: Vec<ObjectDamageOverride>,
    placed_objects: Vec<ObjectPlacementOverride>,
}

impl ChunkModificationLog {
    #[must_use]
    pub const fn new(chunk: ChunkCoord) -> Self {
        Self {
            chunk: SerializableChunkCoord::from_chunk(chunk),
            removed_object_ids: Vec::new(),
            damaged_object_ids: Vec::new(),
            placed_objects: Vec::new(),
        }
    }

    #[must_use]
    pub const fn chunk(&self) -> ChunkCoord {
        self.chunk.to_chunk()
    }

    pub fn record_removed(&mut self, object_id: ObjectId) {
        let raw = object_id.as_u64();
        if !self.removed_object_ids.contains(&raw) {
            self.removed_object_ids.push(raw);
        }
    }

    pub fn record_damaged(&mut self, object_id: ObjectId, damage: u32) {
        let raw = object_id.as_u64();
        if let Some(existing) = self
            .damaged_object_ids
            .iter_mut()
            .find(|entry| entry.object_id == raw)
        {
            existing.damage = damage;
        } else {
            self.damaged_object_ids.push(ObjectDamageOverride {
                object_id: raw,
                damage,
            });
        }
    }

    pub fn record_placed(&mut self, placed: ObjectPlacementOverride) {
        self.placed_objects.push(placed);
    }

    #[must_use]
    pub fn is_removed(&self, object_id: ObjectId) -> bool {
        self.removed_object_ids.contains(&object_id.as_u64())
    }

    #[must_use]
    pub fn damage_for(&self, object_id: ObjectId) -> Option<u32> {
        self.damaged_object_ids
            .iter()
            .find(|entry| entry.object_id == object_id.as_u64())
            .map(|entry| entry.damage)
    }

    #[must_use]
    pub fn visible_generated_objects(
        &self,
        generated_object_ids: impl IntoIterator<Item = ObjectId>,
    ) -> Vec<ObjectId> {
        generated_object_ids
            .into_iter()
            .filter(|object_id| !self.is_removed(*object_id))
            .collect()
    }

    #[must_use]
    pub fn apply_to_generated(
        &self,
        generated_object_ids: impl IntoIterator<Item = ObjectId>,
    ) -> ResolvedChunkObjects {
        ResolvedChunkObjects {
            generated_object_ids: self.visible_generated_objects(generated_object_ids),
            placed_objects: self.placed_objects.clone(),
        }
    }

    pub fn to_save_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_save_json(source: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(source)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct SerializableChunkCoord {
    x: i32,
    z: i32,
}

impl SerializableChunkCoord {
    const fn from_chunk(chunk: ChunkCoord) -> Self {
        Self {
            x: chunk.x,
            z: chunk.z,
        }
    }

    const fn to_chunk(self) -> ChunkCoord {
        ChunkCoord::new(self.x, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct ObjectDamageOverride {
    object_id: u64,
    damage: u32,
}
