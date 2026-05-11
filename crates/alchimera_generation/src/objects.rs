//! Deterministic procedural game-object generation and lifecycle primitives.

use alchimera_core::{ids::ObjectId, seed::WorldSeed};

use crate::{
    biome::{sample_biome, Biome},
    chunk::{ChunkCoord, CHUNK_SIZE_METERS},
    terrain::{sample_height, TerrainConfig},
};

/// Stable prototype key for a procedurally generated object archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectPrototypeKey {
    Tree,
    Boulder,
    Herb,
}

impl ObjectPrototypeKey {
    /// Returns the stable lowercase key used by tooling and visualizers.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::Boulder => "boulder",
            Self::Herb => "herb",
        }
    }
}

/// Metadata for a procedural object archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectPrototype {
    pub key: ObjectPrototypeKey,
    pub display_name: &'static str,
    pub ascii_icon: &'static str,
    pub spawn_weight: u32,
    allowed_biomes: &'static [Biome],
}

/// Returns every object archetype that procedural generation and visualization know about.
#[must_use]
pub const fn object_catalog() -> &'static [ObjectPrototype] {
    &[
        ObjectPrototype {
            key: ObjectPrototypeKey::Tree,
            display_name: "Canopy Tree",
            ascii_icon: "  &&&\n &&&&&\n   |",
            spawn_weight: 5,
            allowed_biomes: &[Biome::Forest, Biome::Grassland],
        },
        ObjectPrototype {
            key: ObjectPrototypeKey::Boulder,
            display_name: "Weathered Boulder",
            ascii_icon: " /\\\n/##\\\n\\##/",
            spawn_weight: 4,
            allowed_biomes: &[Biome::RockyHighland, Biome::RiverValley],
        },
        ObjectPrototype {
            key: ObjectPrototypeKey::Herb,
            display_name: "Wild Herb",
            ascii_icon: " \\|/\n--*--\n /|\\",
            spawn_weight: 7,
            allowed_biomes: &[
                Biome::Grassland,
                Biome::Forest,
                Biome::RiverValley,
                Biome::RockyHighland,
            ],
        },
    ]
}

/// Lifecycle state shared by all procedural game objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    /// The object exists only as deterministic generation data.
    Procedural,
    /// Creation logic has materialized the object into a runtime/world context.
    Created,
    /// The object is active and can participate in simulation.
    Active,
    /// Destruction logic has torn the object down.
    Destroyed,
}

/// Lifecycle transition event emitted by the common object trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLifecycle {
    pub object_id: ObjectId,
    pub previous_state: LifecycleState,
    pub new_state: LifecycleState,
}

/// Common lifecycle and identity contract for generated game objects.
pub trait ProceduralGameObject {
    fn id(&self) -> ObjectId;
    fn prototype_key(&self) -> ObjectPrototypeKey;
    fn lifecycle_state(&self) -> LifecycleState;
    fn set_lifecycle_state(&mut self, state: LifecycleState);

    fn create(&mut self) -> ObjectLifecycle {
        self.transition_to(LifecycleState::Created)
    }

    fn activate(&mut self) -> ObjectLifecycle {
        self.transition_to(LifecycleState::Active)
    }

    fn destroy(&mut self) -> ObjectLifecycle {
        self.transition_to(LifecycleState::Destroyed)
    }

    fn transition_to(&mut self, new_state: LifecycleState) -> ObjectLifecycle {
        let previous_state = self.lifecycle_state();
        self.set_lifecycle_state(new_state);
        ObjectLifecycle {
            object_id: self.id(),
            previous_state,
            new_state,
        }
    }
}

/// Position, rotation, and scale generated for an object instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectTransform {
    pub translation: [f32; 3],
    pub yaw_radians: f32,
    pub scale: f32,
}

/// A generated object instance. Instances are derived from seed/chunk/index, never authored by hand.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedObject {
    id: ObjectId,
    prototype_key: ObjectPrototypeKey,
    chunk: ChunkCoord,
    local_x: f32,
    local_z: f32,
    transform: ObjectTransform,
    lifecycle_state: LifecycleState,
}

impl GeneratedObject {
    #[must_use]
    pub const fn chunk(&self) -> ChunkCoord {
        self.chunk
    }

    #[must_use]
    pub const fn local_position(&self) -> (f32, f32) {
        (self.local_x, self.local_z)
    }

    #[must_use]
    pub const fn transform(&self) -> ObjectTransform {
        self.transform
    }
}

impl ProceduralGameObject for GeneratedObject {
    fn id(&self) -> ObjectId {
        self.id
    }

    fn prototype_key(&self) -> ObjectPrototypeKey {
        self.prototype_key
    }

    fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle_state
    }

    fn set_lifecycle_state(&mut self, state: LifecycleState) {
        self.lifecycle_state = state;
    }
}

/// Stateless deterministic object generator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectGenerator {
    objects_per_chunk: u64,
    terrain_config: TerrainConfig,
}

impl ObjectGenerator {
    #[must_use]
    pub const fn new(objects_per_chunk: u64, terrain_config: TerrainConfig) -> Self {
        Self {
            objects_per_chunk,
            terrain_config,
        }
    }

    /// Generate all objects for a chunk from procedural inputs.
    #[must_use]
    pub fn generate_chunk(self, seed: WorldSeed, chunk: ChunkCoord) -> Vec<GeneratedObject> {
        (0..self.objects_per_chunk)
            .map(|index| self.generate_object(seed, chunk, index))
            .collect()
    }

    fn generate_object(self, seed: WorldSeed, chunk: ChunkCoord, index: u64) -> GeneratedObject {
        let instance_seed = seed.derive_child("object.instance", &[chunk.x, chunk.z], index);
        let local_x = unit(instance_seed, "object.local_x") * CHUNK_SIZE_METERS;
        let local_z = unit(instance_seed, "object.local_z") * CHUNK_SIZE_METERS;
        let biome = sample_biome(seed, chunk, local_x, local_z);
        let prototype = choose_prototype(instance_seed, biome);
        let height = sample_height(seed, chunk, local_x, local_z, self.terrain_config);
        let bounds = chunk.world_bounds();

        GeneratedObject {
            id: ObjectId::from_seed_chunk_and_index(seed, [chunk.x, chunk.z], index),
            prototype_key: prototype.key,
            chunk,
            local_x,
            local_z,
            transform: ObjectTransform {
                translation: [bounds.min_x + local_x, height, bounds.min_z + local_z],
                yaw_radians: unit(instance_seed, "object.yaw") * std::f32::consts::TAU,
                scale: 0.75 + unit(instance_seed, "object.scale") * 0.75,
            },
            lifecycle_state: LifecycleState::Procedural,
        }
    }
}

impl Default for ObjectGenerator {
    fn default() -> Self {
        Self::new(8, TerrainConfig::default())
    }
}

fn choose_prototype(seed: WorldSeed, biome: Biome) -> &'static ObjectPrototype {
    let candidates: Vec<_> = object_catalog()
        .iter()
        .filter(|prototype| prototype.allowed_biomes.contains(&biome))
        .collect();
    let total_weight: u32 = candidates
        .iter()
        .map(|prototype| prototype.spawn_weight)
        .sum();
    let mut ticket = (unit(seed, "object.prototype") * total_weight as f32).floor() as u32;

    for prototype in candidates {
        if ticket < prototype.spawn_weight {
            return prototype;
        }
        ticket -= prototype.spawn_weight;
    }

    &object_catalog()[0]
}

fn unit(seed: WorldSeed, label: &str) -> f32 {
    let child = seed.derive_child(label, &[], 0).as_u64();
    let unit = (child >> 11) as f64 / ((1_u64 << 53) as f64);
    unit as f32
}
