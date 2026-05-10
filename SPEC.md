# Alchimera Game Specification

**Working title:** Alchimera  
**Genre:** 3D open-world survival sandbox  
**Engine:** Bevy  
**Language:** Rust  
**Core pitch:** A Minecraft-like procedural open-world game where the world is built from editable, simulated, procedurally generated objects instead of cubic blocks.

---

## 1. Vision

Alchimera is a first-person / third-person open-world sandbox where players explore, harvest, craft, build, and survive in a fully procedural world. Unlike voxel/block games, the world is composed of natural and crafted objects: stones, boulders, logs, branches, plants, terrain patches, ore veins, ruins, machines, furniture, walls, roofs, doors, tools, and creatures.

The design goal is to preserve the systemic freedom of Minecraft while replacing the visible grid of blocks with organic, mesh-based objects that feel handcrafted but are generated deterministically.

The player should feel like they are modifying a real wilderness rather than mining cubes.

---

## 2. Core Requirements

### 2.1 Mandatory Technical Requirements

- The game must be written in **Rust**.
- The game must use the **Bevy engine** as the primary engine/framework.
- The game must be 3D.
- The game must support a procedurally generated open world.
- The world must not be represented primarily as visible Minecraft-style cubes.
- The world must be composed of procedural object instances and generated meshes.
- Generation must be deterministic from a world seed.
- The architecture must be modular enough to support future multiplayer.

### 2.2 Target Bevy Stack

Use Bevy's ECS as the central architecture.

Recommended dependencies:

- `bevy`: engine, ECS, rendering, input, assets.
- `bevy_rapier3d`: physics, collision, character movement, rigid bodies.
- `noise` or `fastnoise-lite`: terrain, biome, and resource noise.
- `rand`, `rand_chacha`: deterministic seeded generation.
- `serde`, `ron` or `toml`: data-driven definitions.
- `bevy_asset_loader`: structured loading states.
- `bevy_kira_audio` or Bevy audio: music and sound effects.
- `leafwing-input-manager`: configurable input actions.
- `iyes_progress` or custom loading progress: loading screens.

Optional later dependencies:

- `bevy_ecs_tilemap`: only for hidden low-resolution planning maps, not visible block world geometry.
- `big-brain` or custom utility AI: creature behavior.
- `renet` / `bevy_replicon`: multiplayer networking.

---

## 3. Design Pillars

### 3.1 Organic Procedural Objects

The world is made of objects rather than blocks. Objects have real shapes, scale variation, material properties, and gameplay affordances.

Examples:

- Trees are composed of trunks, branches, leaves, fruit, roots, bark, knots, and hollow sections.
- Rocks are irregular meshes with mineral deposits, cracks, hardness, and break points.
- Terrain is continuous mesh terrain with cliffs, caves, slopes, paths, soil, grass, and erosion patterns.
- Player buildings use structural pieces such as beams, planks, stone slabs, walls, shingles, pillars, ropes, and panels.
- Resource nodes are embedded object clusters rather than ore blocks.

### 3.2 Minecraft-Like Freedom Without Cubes

The player can:

- Explore a large procedural world.
- Gather resources.
- Craft tools and structures.
- Build custom bases.
- Dig, mine, cut, burn, plant, farm, and reshape the environment.
- Encounter emergent systems.

But interaction happens through object operations:

- Chop branches off a tree.
- Split logs into planks.
- Chip a boulder into stones.
- Excavate terrain volumes.
- Place modular structural pieces.
- Combine objects into assemblies.

### 3.3 Simulation First

Objects should expose systemic properties:

- Material type.
- Mass.
- Durability.
- Hardness.
- Flammability.
- Moisture.
- Temperature.
- Growth stage.
- Structural support.
- Resource yield.

These properties drive crafting, destruction, physics, fire, weathering, and ecology.

### 3.4 Deterministic, Streamed Open World

The world is split into simulation chunks for streaming, saving, and generation. Chunks are not visible cube chunks; they are invisible spatial partitions containing terrain mesh data and object instances.

---

## 4. Player Fantasy

The player is an alchemist-explorer arriving in a strange living world. They discover materials, transform objects, build shelters, craft tools, and uncover procedural ruins and ecosystems.

Alchemy gives the game an identity beyond Minecraft:

- Materials can be transmuted.
- Plants, ores, and essences have hidden properties.
- Crafting is experimental.
- The player can alter the world through physical and magical processes.

---

## 5. World Model

### 5.1 World Seed

Every world is generated from a seed.

The seed controls:

- Terrain heightfields.
- Continental layout.
- Biomes.
- Rivers and lakes.
- Cave networks.
- Object distribution.
- Ruins.
- Creature spawns.
- Resource clusters.

The same seed and version should produce the same base world.

### 5.2 Spatial Partitioning

Use invisible square chunks for streaming and persistence.

Initial recommendation:

- Chunk size: `64m x 64m` horizontal area.
- Vertical range: variable, based on terrain/cave requirements.
- Active radius: 3-6 chunks around player for prototype.
- Each chunk stores terrain mesh plus object instances.

Chunk data should include:

```rust
struct ChunkCoord {
    x: i32,
    z: i32,
}

struct WorldChunk {
    coord: ChunkCoord,
    seed: u64,
    terrain: TerrainChunkData,
    objects: Vec<ObjectInstance>,
    modifications: ChunkModificationLog,
}
```

### 5.3 Terrain Representation

The terrain should be continuous mesh terrain, not blocks.

Phase 1 terrain:

- Heightmap-based terrain mesh.
- Procedural normal generation.
- Biome-based materials.
- Collision mesh generated per chunk.

Phase 2 terrain:

- Editable terrain using signed distance fields, dual contouring, or marching cubes.
- Cave generation.
- Digging and excavation.
- Terrain deformation persistence.

Recommended approach:

- Start with heightmap terrain for MVP.
- Design APIs so terrain generation can later move to SDF without rewriting gameplay systems.

### 5.4 Object Instances

Everything interactable in the world should be an object instance.

```rust
struct ObjectInstance {
    id: ObjectId,
    prototype_id: PrototypeId,
    transform: Transform,
    seed: u64,
    state: ObjectState,
    components: ObjectComponentSet,
}
```

Objects should be generated from prototypes.

Prototype examples:

- `tree.oak.young`
- `tree.oak.mature`
- `rock.granite.small`
- `rock.granite.boulder`
- `plant.berry_bush.red`
- `ore.copper.surface_vein`
- `ruin.wall_fragment.stone`
- `building.wood_beam`
- `crafting.workbench.basic`

### 5.5 Procedural Object Generation

Object generation should combine:

- Prototype definition.
- Instance seed.
- Biome context.
- Local slope and altitude.
- Nearby objects.
- Simulation state.

Each generated object may produce:

- Mesh.
- Collider.
- Material assignments.
- Attachment points.
- Loot/resource profile.
- Interaction affordances.
- Child objects.

Example:

```rust
trait ProceduralObjectGenerator {
    fn generate(&self, context: &ObjectGenerationContext) -> GeneratedObject;
}
```

---

## 6. Procedural Objects Instead of Blocks

### 6.1 Object Categories

#### Natural Objects

- Trees.
- Bushes.
- Grass clusters.
- Mushrooms.
- Flowers.
- Logs.
- Stones.
- Boulders.
- Ore veins.
- Crystals.
- Bones.
- Shells.
- Nests.
- Fallen branches.

#### Terrain Objects

- Cliffs.
- Cave mouths.
- Soil patches.
- Sand dunes.
- River banks.
- Gravel beds.
- Mud patches.
- Snow drifts.

#### Constructed Objects

- Beams.
- Planks.
- Stone slabs.
- Walls.
- Roof panels.
- Doors.
- Windows.
- Fences.
- Stairs.
- Ladders.
- Bridges.
- Machines.
- Workbenches.
- Furnaces.
- Storage containers.

#### Alchemical Objects

- Distillers.
- Crucibles.
- Retorts.
- Essence collectors.
- Transmutation circles.
- Crystal lenses.
- Infusion vats.
- Living metal growths.

### 6.2 Object Composition

Complex objects should be composed hierarchically.

Example tree:

- Root object: `OakTree`
  - Trunk segments.
  - Branch segments.
  - Leaf clusters.
  - Fruit objects.
  - Nest attachment points.
  - Fungal growth overlays.

A tree is not one block and not necessarily one mesh. It is a generated assembly of related components.

### 6.3 Object Damage and Harvesting

Object interactions should use tools and material properties.

Examples:

- Axe + tree branch = branch detaches and becomes pickup/log.
- Pickaxe + boulder = chips off stone fragments until boulder fractures.
- Shovel + soil patch = produces soil clumps and terrain depression.
- Hammer + ore vein = extracts ore pieces.
- Fire + dry bush = spreads flame and leaves ash object.

Objects can transition through states:

```rust
enum ObjectState {
    Intact,
    Damaged { damage: f32 },
    Fractured,
    Harvested,
    Burning { heat: f32 },
    Ash,
    Growing { age: f32 },
    Dormant,
}
```

---

## 7. Gameplay Systems

### 7.1 Exploration

The player explores biomes, caves, ruins, and resource sites.

Required MVP features:

- First-person camera.
- Walk, run, jump, crouch.
- Interact raycast.
- Basic inventory.
- Basic resource gathering.
- Basic crafting.
- Procedural terrain and object spawning.

### 7.2 Survival

Survival systems should be light at first, deeper later.

MVP:

- Health.
- Stamina.
- Tool durability.

Later:

- Hunger.
- Thirst.
- Temperature.
- Weather exposure.
- Sleep.
- Injury types.

### 7.3 Crafting

Crafting should be object and material based.

Recipes can specify material classes instead of exact items.

Example:

```ron
Recipe(
    id: "tool.stone_axe",
    inputs: [
        MaterialClass("stone", amount: 1),
        MaterialClass("wood_handle", amount: 1),
        MaterialClass("binding", amount: 1),
    ],
    output: Item("stone_axe", amount: 1),
)
```

Crafting stations:

- Hand crafting.
- Workbench.
- Furnace/kiln.
- Alchemy table.
- Crucible.
- Loom.
- Mason bench.

### 7.4 Building

Building must avoid block placement as the primary mechanic.

Instead, use modular object placement:

- Snap beams to attachment points.
- Place planks across beams.
- Fit wall panels into frames.
- Stack stones into walls with procedural variation.
- Stretch ropes between anchors.
- Place roof shingles over rafters.

MVP building should support:

- Place campfire.
- Place workbench.
- Place storage chest.
- Place simple wooden beam.
- Place simple wall panel.

Later building should support:

- Structural stability.
- Procedural wall/roof/floor assemblies.
- Decorative object placement.
- Blueprint mode.

### 7.5 Alchemy

Alchemy differentiates Alchimera from generic survival sandboxes.

Core concepts:

- Materials have hidden alchemical traits.
- Traits can be discovered through experimentation.
- Alchemical processes transform material state.
- Some world objects respond to alchemical energy.

Example traits:

- `volatile`
- `binding`
- `aerial`
- `aqueous`
- `verdant`
- `metallic`
- `luminous`
- `corrosive`
- `stabilizing`

Example processes:

- Distill.
- Calcify.
- Dissolve.
- Infuse.
- Ferment.
- Transmute.
- Crystallize.

MVP alchemy:

- Discover material traits.
- Combine two ingredients at a basic alchemy table.
- Produce simple potions or crafting reagents.

---

## 8. World Generation

### 8.1 Generation Pipeline

For each chunk:

1. Compute terrain base height.
2. Compute biome map.
3. Generate terrain mesh.
4. Generate terrain material layers.
5. Generate water bodies if applicable.
6. Place large anchors: cliffs, caves, ruins, large trees, boulders.
7. Place medium objects: trees, rocks, shrubs, resource nodes.
8. Place small detail objects: grass, flowers, sticks, mushrooms.
9. Generate colliders for relevant objects.
10. Spawn Bevy entities for active objects.
11. Save only player modifications and non-deterministic state changes.

### 8.2 Biomes

Initial biomes:

- Temperate forest.
- Meadow.
- Rocky hills.
- River valley.
- Marsh.

Later biomes:

- Snow forest.
- Desert badlands.
- Crystal caverns.
- Ash wastes.
- Giant mushroom forest.
- Floating islands.

### 8.3 Resources

Resources should appear as natural objects.

Examples:

- Copper appears as green-blue streaks in exposed rocks.
- Iron appears as red-brown nodules in cliffs.
- Clay appears in river banks.
- Herbs appear in biome-specific plant clusters.
- Crystal appears in caves or magical anomalies.

### 8.4 Ruins and Points of Interest

POIs should be generated as object assemblies.

Examples:

- Collapsed stone tower.
- Abandoned alchemy hut.
- Root-covered shrine.
- Broken bridge.
- Cave altar.
- Meteor crater.

POIs must support:

- Deterministic placement.
- Loot containers.
- Lore fragments.
- Hostile or neutral creature spawns.
- Unique alchemical materials.

---

## 9. ECS Architecture

### 9.1 Bevy App Structure

Recommended plugin layout:

```rust
pub struct AlchimeraPlugin;

impl Plugin for AlchimeraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ObjectPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(CraftingPlugin)
            .add_plugins(BuildingPlugin)
            .add_plugins(AlchemyPlugin)
            .add_plugins(UiPlugin);
    }
}
```

### 9.2 Suggested Crate Modules

```text
src/
  main.rs
  app.rs
  states.rs
  world/
    mod.rs
    chunk.rs
    generation.rs
    terrain.rs
    biome.rs
    streaming.rs
    persistence.rs
  objects/
    mod.rs
    prototype.rs
    instance.rs
    generation.rs
    interaction.rs
    damage.rs
  player/
    mod.rs
    controller.rs
    camera.rs
    input.rs
    stats.rs
  inventory/
    mod.rs
    item.rs
    stack.rs
  crafting/
    mod.rs
    recipe.rs
    station.rs
  building/
    mod.rs
    placement.rs
    snapping.rs
    stability.rs
  alchemy/
    mod.rs
    traits.rs
    process.rs
    discovery.rs
  rendering/
    mod.rs
    materials.rs
    mesh_generation.rs
  physics/
    mod.rs
    collision.rs
  ui/
    mod.rs
    hud.rs
    inventory_ui.rs
```

### 9.3 App States

```rust
enum AppState {
    Boot,
    LoadingAssets,
    MainMenu,
    CreatingWorld,
    InGame,
    Paused,
}
```

### 9.4 Core Components

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct WorldObject {
    id: ObjectId,
    prototype_id: PrototypeId,
}

#[derive(Component)]
struct Harvestable {
    tool_kind: ToolKind,
    yields: Vec<ResourceYield>,
}

#[derive(Component)]
struct Durability {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct MaterialProperties {
    material: MaterialId,
    hardness: f32,
    flammability: f32,
    density: f32,
}

#[derive(Component)]
struct ProceduralMeshSource {
    seed: u64,
    generator: GeneratorId,
}
```

---

## 10. Data-Driven Definitions

Use data files for prototypes, recipes, materials, and biomes.

Recommended location:

```text
assets/data/
  materials/
  objects/
  recipes/
  biomes/
  alchemy/
```

Example material definition:

```ron
MaterialDef(
    id: "wood.oak",
    display_name: "Oak Wood",
    hardness: 0.35,
    density: 0.6,
    flammability: 0.8,
    alchemy_traits: ["verdant", "binding"],
)
```

Example object prototype:

```ron
ObjectPrototype(
    id: "tree.oak.mature",
    generator: "tree_lsystem_v1",
    materials: ["wood.oak", "leaf.oak"],
    interaction_tags: ["choppable", "climbable"],
    base_durability: 120.0,
)
```

---

## 11. Rendering and Art Direction

### 11.1 Visual Style

Recommended style: stylized naturalism.

Rationale:

- Procedural meshes look better when stylized than when aiming for photorealism.
- Lower asset fidelity keeps development feasible.
- Strong silhouettes make object interaction readable.

Visual references:

- Organic shapes.
- Soft gradients.
- Chunky handcrafted silhouettes.
- Readable materials.
- Warm fantasy alchemy palette.

### 11.2 Mesh Generation Requirements

Generated meshes should support:

- LOD levels.
- Collision simplification.
- Stable deterministic output.
- Material slots.
- Optional vertex colors.
- Bounds for culling.

### 11.3 Performance Targets

Prototype target:

- 60 FPS at 1080p on mid-range desktop GPU.
- 3-6 chunk active radius.
- Up to 5,000 lightweight detail objects near player.
- Up to 500 interactable objects active near player.

Optimization requirements:

- Use instancing for repeated detail meshes.
- Despawn distant object entities while retaining deterministic data.
- Use LOD for trees, rocks, and ruins.
- Generate heavy meshes asynchronously where possible.
- Avoid spawning physics colliders for distant decorative objects.

---

## 12. Physics and Interaction

### 12.1 Physics

Use physics for:

- Player collision.
- Ground detection.
- Interact raycasts.
- Pickups.
- Throwable objects.
- Structure collision.

Do not simulate every object as a rigid body by default. Most world objects should be static colliders until interacted with.

### 12.2 Interaction Model

The player can target objects with a raycast.

Interaction flow:

1. Player looks at object.
2. UI shows object name and available action.
3. Player presses interact or uses equipped tool.
4. System checks tool, material, durability, and action rules.
5. Object state changes.
6. Resources, particles, sound, and animation are spawned.

Example actions:

- Pick up.
- Chop.
- Mine.
- Cut.
- Dig.
- Place.
- Open.
- Ignite.
- Extinguish.
- Inspect.
- Transmute.

---

## 13. Inventory and Items

### 13.1 Inventory

MVP inventory:

- Fixed-size slot inventory.
- Stackable items.
- Hotbar.
- Equipped item/tool.

Later:

- Weight limits.
- Containers.
- Item quality.
- Material variants.

### 13.2 Item Types

- Resources.
- Tools.
- Food.
- Potions.
- Building pieces.
- Crafting stations.
- Lore items.

### 13.3 Material Variants

Items should preserve material identity when useful.

Example:

- `plank` made from oak differs from `plank` made from pine.
- `stone_chunk` made from granite differs from limestone.
- Alchemical traits depend on source material.

---

## 14. Persistence

### 14.1 Save Model

Do not save the entire generated world. Save:

- World seed.
- Player state.
- Inventory.
- Modified chunks.
- Destroyed/harvested/generated object overrides.
- Placed player objects.
- Dynamic entities that matter.

### 14.2 Chunk Modification Log

Each chunk should store a compact modification log.

Examples:

- Object removed.
- Object damaged.
- Object transformed.
- Player object placed.
- Terrain edited.
- Container inventory changed.

This keeps saves smaller and allows deterministic regeneration of unchanged world content.

---

## 15. Multiplayer Readiness

Multiplayer is not required for MVP, but architecture must not block it.

Guidelines:

- Keep gameplay logic deterministic where possible.
- Separate input from simulation actions.
- Use stable object IDs.
- Avoid direct resource mutation without events/commands.
- Store world changes as commands or events.
- Keep save state serializable.

Potential future networking model:

- Server authoritative world simulation.
- Clients stream chunks and object states.
- Client-side prediction for player movement.
- Replicated object modifications and inventory actions.

---

## 16. MVP Scope

### 16.1 MVP Goal

Build a playable vertical slice where the player can walk through a procedural world, interact with procedural objects, gather materials, craft simple items, and place basic structures.

### 16.2 MVP Features

Required:

- Bevy app bootstrapping.
- First-person player controller.
- Procedural heightmap terrain chunks.
- Chunk streaming around player.
- Deterministic seed-based generation.
- Procedural tree objects.
- Procedural rock objects.
- Procedural plant/detail objects.
- Object raycast interaction.
- Harvest trees and rocks.
- Inventory and hotbar.
- Craft stone axe.
- Place workbench.
- Place basic beam/wall object.
- Save/load player inventory and chunk modifications.

Nice-to-have:

- Basic day/night cycle.
- Basic ambient audio.
- Simple alchemy table.
- Simple creature.

Explicitly out of MVP:

- Multiplayer.
- Infinite terrain deformation.
- Advanced cave systems.
- Full structural simulation.
- Complex AI ecology.
- Full quest/lore system.

---

## 17. Milestones

### Milestone 0: Project Bootstrap

- Create Rust Bevy project.
- Add core dependencies.
- Add app states.
- Add plugin structure.
- Display empty 3D scene.

Acceptance criteria:

- `cargo run` opens a Bevy window.
- A camera and light exist.
- The app can enter `InGame` state.

### Milestone 1: Player Prototype

- Add first-person controller.
- Add gravity and collision.
- Add input actions.
- Add interact raycast debug line.

Acceptance criteria:

- Player can move, look, jump, and collide with ground.
- Looking at an object reports a hit.

### Milestone 2: Terrain Chunks

- Generate terrain chunks from seed.
- Stream chunks around player.
- Add terrain materials and collision.

Acceptance criteria:

- Player can walk across generated terrain.
- Terrain is deterministic for same seed.
- Chunks spawn/despawn as player moves.

### Milestone 3: Procedural Objects

- Add object prototype registry.
- Add procedural tree generator.
- Add procedural rock generator.
- Spawn objects deterministically per chunk.

Acceptance criteria:

- Trees and rocks appear in sensible locations.
- Same seed produces same object positions.
- Objects have colliders and names.

### Milestone 4: Harvesting and Inventory

- Add harvestable components.
- Add tool actions.
- Add inventory and hotbar.
- Add resource drops.

Acceptance criteria:

- Player can harvest wood and stone.
- Items appear in inventory.
- Object state changes persist until reload.

### Milestone 5: Crafting and Building

- Add recipe registry.
- Add hand crafting.
- Add workbench placement.
- Add simple build placement mode.

Acceptance criteria:

- Player can craft a stone axe.
- Player can place a workbench.
- Player can place a beam or wall panel.

### Milestone 6: Saving and Loading

- Save world seed, player state, inventory, and chunk modifications.
- Load saved state.

Acceptance criteria:

- Harvested objects stay harvested after reload.
- Placed objects remain after reload.
- Inventory persists.

### Milestone 7: Alchemy Vertical Slice

- Add alchemical traits to materials.
- Add alchemy table.
- Add simple two-ingredient experiment.
- Add discovery UI.

Acceptance criteria:

- Player can discover a material trait.
- Player can combine ingredients into a reagent or potion.

---

## 18. Testing Strategy

### 18.1 Unit Tests

Unit-test pure logic:

- Seed hashing.
- Chunk coordinate conversion.
- Biome selection.
- Object placement determinism.
- Recipe matching.
- Inventory stacking.
- Save/load serialization.

### 18.2 Integration Tests

Integration-test systems where possible:

- Chunk generation produces expected object counts.
- Harvest action changes object state and inventory.
- Crafting consumes inputs and creates output.
- Save/load round trip preserves modifications.

### 18.3 Manual Playtest Checklist

- Start new world.
- Walk around without falling through terrain.
- Confirm terrain streams without visible stalls.
- Look at tree and see interaction prompt.
- Harvest wood.
- Harvest stone.
- Craft stone axe.
- Place workbench.
- Save game.
- Quit and reload.
- Confirm inventory and world changes persisted.

---

## 19. Risk Register

### Risk: Procedural object generation becomes too expensive

Mitigation:

- Use LOD.
- Cache generated meshes.
- Generate asynchronously.
- Use simple prototype generators first.

### Risk: Non-block building is hard to make intuitive

Mitigation:

- Start with snap points and ghost previews.
- Keep MVP building pieces simple.
- Add blueprint mode later.

### Risk: Editable continuous terrain is complex

Mitigation:

- Start with non-editable heightmap terrain.
- Represent digging as object/resource harvesting first.
- Add SDF terrain after core loop works.

### Risk: Bevy API changes

Mitigation:

- Pin Bevy version.
- Keep engine-facing code isolated in plugins.
- Avoid depending on unstable internals.

### Risk: Save files become large

Mitigation:

- Save only modifications.
- Regenerate unchanged deterministic content.
- Use compact IDs and logs.

---

## 20. Open Questions

- Should the default perspective be first-person only, or toggle first/third person?
- How realistic should object destruction be: health bars, fracture points, or physics slicing?
- Should terrain deformation be a core feature or a post-MVP feature?
- Should alchemy be magical, scientific, or a hybrid aesthetic?
- Should combat be important, optional, or minimal?
- What visual style should be targeted: cozy stylized, dark survival, or high fantasy?

---

## 21. Definition of Done for the Spec

This specification is complete enough when a developer can:

- Create the Bevy/Rust project structure.
- Implement deterministic terrain chunks.
- Implement procedural object spawning.
- Build a playable gathering/crafting/building MVP.
- Extend the project toward alchemy, persistence, and multiplayer without rewriting the core architecture.
