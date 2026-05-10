# Parallel Subagent Task Breakdown

> **For Hermes:** Use `subagent-driven-development` to execute these tasks. Each implementation task must follow `IMPLEMENTATION_GUIDELINES.md`: isolated worktree, strict TDD, targeted tests, performance measurement where relevant, spec review, then quality/performance review.

**Goal:** Convert `SPEC.md` and `IMPLEMENTATION_GUIDELINES.md` into an execution plan that maximizes safe parallel work by assigning non-overlapping file ownership to subagents.

**Architecture:** Build Alchimera as a Rust/Bevy workspace with core pure logic crates separated from Bevy runtime adapters. Pure logic tasks are parallelized aggressively; integration tasks are serialized through narrow merge gates.

**Tech Stack:** Rust, Bevy, Bevy ECS, Criterion, Serde/RON, deterministic RNG, optional Rapier later.

---

## 0. Parallelization Strategy

### 0.1 Key Rule

Maximize parallelism by splitting work along crate/module boundaries and data ownership boundaries:

- `alchimera_core`: IDs, seed math, inventory, crafting, save data, shared types.
- `alchimera_generation`: chunk math, biome/terrain/object placement, mesh data, benchmarks.
- `alchimera_game`: Bevy app, plugins, ECS systems, rendering adapters, diagnostics.
- `assets/data`: data definitions and fixtures.
- `docs`: plans, ADRs, profiling reports, playtest templates.

### 0.2 Merge Model

Use waves. Within a wave, tasks can run in parallel. Between waves, merge into an integration branch and run full checks.

Recommended integration branches:

```text
main
integration/wave-0-bootstrap
integration/wave-1-core-foundations
integration/wave-2-generation-foundations
integration/wave-3-gameplay-foundations
integration/wave-4-vertical-slice
```

### 0.3 Worktree Convention

Each task gets a dedicated worktree:

```bash
mkdir -p /home/openclaw/worktrees/alchimera

git worktree add \
  -b <branch> \
  /home/openclaw/worktrees/alchimera/<task-id>
```

Subagents must work only inside their assigned worktree.

### 0.4 Standard Commands

Every implementation subagent runs at least:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If task is performance-sensitive, also run targeted benchmark:

```bash
cargo bench --bench <bench_name>
```

### 0.5 Review Gates

Every task has these gates:

1. **Implementation gate:** Subagent reports tests, commands, files changed, commit hash.
2. **Spec gate:** Reviewer checks task against `SPEC.md` and this plan.
3. **Quality/performance gate:** Reviewer checks Rust/Bevy quality, test quality, determinism, and performance evidence.
4. **Integration gate:** Orchestrator merges only after both reviews pass.

---

## 1. Wave Overview

### Wave 0: Repository Bootstrap — mostly serial, short

Purpose: create the workspace skeleton so later tasks can safely parallelize.

Parallelism: low. Run W0-T1 first. W0-T2 through W0-T5 can then run in parallel.

### Wave 1: Core Foundations — high parallelism

Purpose: pure tested types and rules that many systems depend on.

Parallelism: high. Most tasks modify separate modules in `alchimera_core`.

### Wave 2: Generation Foundations — high parallelism after core IDs/seed/chunk land

Purpose: deterministic terrain, biome, object placement, mesh data, and benchmarks.

Parallelism: high, split by generator subsystem.

### Wave 3: Bevy Runtime Foundations — medium parallelism

Purpose: Bevy app shell, state/plugins, diagnostics, player, streaming adapter.

Parallelism: medium. Avoid multiple agents touching app registration files simultaneously.

### Wave 4: Gameplay Vertical Slice — medium/high parallelism

Purpose: harvest, inventory UI, crafting, placement, persistence integration.

Parallelism: medium. Pure gameplay logic can parallelize; ECS integration must be carefully serialized.

### Wave 5: Polish and Integration — low/medium parallelism

Purpose: final vertical-slice pass, profiling, docs, playtest, bug fixes.

Parallelism: low for integration, medium for docs/profiling/playtest branches.

---

## 2. Dependency Graph

```text
W0-T1 workspace bootstrap
  ├─ W0-T2 CI/local command docs
  ├─ W0-T3 crate module skeletons
  ├─ W0-T4 test/benchmark harness
  └─ W0-T5 asset data directories

W0 complete
  ├─ W1-T1 seed derivation
  ├─ W1-T2 chunk coord math
  ├─ W1-T3 stable ID newtypes
  ├─ W1-T4 material/item IDs
  ├─ W1-T5 inventory stack logic
  ├─ W1-T6 recipe matching logic
  ├─ W1-T7 save data version shell
  └─ W1-T8 error types and validation helpers

W1 seed + chunk + IDs complete
  ├─ W2-T1 biome selection
  ├─ W2-T2 height sampling
  ├─ W2-T3 terrain mesh data
  ├─ W2-T4 object prototype schema
  ├─ W2-T5 object placement
  ├─ W2-T6 tree generator data
  ├─ W2-T7 rock generator data
  ├─ W2-T8 generation benchmarks
  └─ W2-T9 chunk modification log

W0 game skeleton + W1 foundations complete
  ├─ W3-T1 Bevy app state shell
  ├─ W3-T2 diagnostics overlay
  ├─ W3-T3 player controller skeleton
  ├─ W3-T4 camera/input actions
  ├─ W3-T5 terrain rendering adapter
  ├─ W3-T6 object spawning adapter
  └─ W3-T7 chunk streaming resource/systems

W1 inventory/crafting + W2 objects + W3 runtime complete
  ├─ W4-T1 interact raycast events
  ├─ W4-T2 harvesting ECS system
  ├─ W4-T3 inventory hotbar model/UI shell
  ├─ W4-T4 hand crafting ECS system
  ├─ W4-T5 placement ghost/build command
  ├─ W4-T6 save/load integration
  └─ W4-T7 alchemy trait core slice
```

---

## 3. Wave 0 Tasks: Bootstrap

### W0-T1: Create Rust/Bevy Workspace Skeleton

**Parallel safety:** Must run first. Serial task.

**Worktree:** `/home/openclaw/worktrees/alchimera/w0-t1-workspace`  
**Branch:** `feat/workspace-bootstrap`

**Objective:** Create a compiling Rust workspace with Bevy-ready binary and empty library crates.

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `crates/alchimera_core/Cargo.toml`
- Create: `crates/alchimera_core/src/lib.rs`
- Create: `crates/alchimera_generation/Cargo.toml`
- Create: `crates/alchimera_generation/src/lib.rs`
- Create: `crates/alchimera_game/Cargo.toml`
- Create: `crates/alchimera_game/src/lib.rs`

**TDD:**
- RED: Add a minimal compile test or crate smoke test that initially fails because crates do not exist.
- GREEN: Create workspace and smoke tests.

**Commands:**
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -- --help || true
```

**Acceptance criteria:**
- Workspace compiles.
- All crates are addressable with `cargo test --workspace`.
- Root binary exists and can start without game content.

**Commit:** `feat: bootstrap rust workspace`

---

### W0-T2: Add Developer README Commands

**Parallel safety:** Can run after W0-T1. Docs only.

**Worktree:** `/home/openclaw/worktrees/alchimera/w0-t2-readme`  
**Branch:** `docs/developer-commands`

**Objective:** Document canonical local commands for build, test, lint, bench, run, and worktrees.

**Files:**
- Create/Modify: `README.md`
- Modify: `IMPLEMENTATION_GUIDELINES.md` only if commands need correction.

**TDD:** Not applicable for docs, but verify commands are syntactically accurate.

**Commands:**
```bash
cargo fmt --all -- --check
cargo test --workspace
```

**Acceptance criteria:**
- README has a “Development Commands” section.
- README references `SPEC.md`, `IMPLEMENTATION_GUIDELINES.md`, and this plan.
- Worktree example is included.

**Commit:** `docs: add developer workflow commands`

---

### W0-T3: Add Module Skeletons Without Behavior

**Parallel safety:** Can run after W0-T1. Avoid if W0-T1 still changing crate roots.

**Worktree:** `/home/openclaw/worktrees/alchimera/w0-t3-modules`  
**Branch:** `feat/module-skeletons`

**Objective:** Create empty modules matching the spec so later tasks can own individual files.

**Files:**
- Modify: `crates/alchimera_core/src/lib.rs`
- Create: `crates/alchimera_core/src/{ids,seed,inventory,crafting,save,error}.rs`
- Modify: `crates/alchimera_generation/src/lib.rs`
- Create: `crates/alchimera_generation/src/{chunk,biome,terrain,mesh,objects,modification_log}.rs`
- Modify: `crates/alchimera_game/src/lib.rs`
- Create: `crates/alchimera_game/src/{states,world,player,objects,ui,diagnostics}.rs`

**TDD:**
- Add smoke tests that each public module can be referenced.

**Commands:**
```bash
cargo test --workspace module_smoke -- --nocapture
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

**Acceptance criteria:**
- Modules exist but contain no premature behavior.
- Public API is minimal.

**Commit:** `feat: add module skeletons`

---

### W0-T4: Add Test and Benchmark Harness

**Parallel safety:** Can run after W0-T1. May touch root `Cargo.toml`; coordinate with W0-T1 only.

**Worktree:** `/home/openclaw/worktrees/alchimera/w0-t4-harness`  
**Branch:** `test/benchmark-harness`

**Objective:** Add Criterion benchmark infrastructure and a trivial benchmark to prove `cargo bench` works.

**Files:**
- Modify: `Cargo.toml`
- Create: `benches/smoke.rs`
- Create: `docs/profiling/README.md`

**TDD:**
- RED: Run `cargo bench --bench smoke` before benchmark exists and observe failure.
- GREEN: Add smoke benchmark.

**Commands:**
```bash
cargo bench --bench smoke
cargo test --workspace
cargo fmt --all -- --check
```

**Acceptance criteria:**
- `cargo bench --bench smoke` runs.
- Profiling docs explain where benchmark reports go.

**Commit:** `test: add benchmark harness`

---

### W0-T5: Add Asset Data Directory and Validation Fixtures

**Parallel safety:** Can run after W0-T1. Owns only `assets/data` and fixture docs.

**Worktree:** `/home/openclaw/worktrees/alchimera/w0-t5-assets`  
**Branch:** `feat/asset-data-layout`

**Objective:** Create the data directory layout and sample placeholder files for materials, objects, recipes, biomes, and alchemy.

**Files:**
- Create: `assets/data/materials/.gitkeep`
- Create: `assets/data/objects/.gitkeep`
- Create: `assets/data/recipes/.gitkeep`
- Create: `assets/data/biomes/.gitkeep`
- Create: `assets/data/alchemy/.gitkeep`
- Create: `assets/data/README.md`

**TDD:** Docs/data layout only; verify paths exist.

**Commands:**
```bash
cargo test --workspace
```

**Acceptance criteria:**
- Directory layout matches `SPEC.md` and `IMPLEMENTATION_GUIDELINES.md`.
- README explains data ownership and validation expectations.

**Commit:** `feat: add asset data layout`

---

## 4. Wave 1 Tasks: Core Foundations

> Wave 1 can run with high parallelism after Wave 0 integration. Each task owns a separate `alchimera_core` module.

### W1-T1: WorldSeed and Deterministic Seed Derivation

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t1-seed`  
**Branch:** `feat/world-seed`

**Objective:** Add `WorldSeed` newtype and deterministic child seed derivation.

**Files:**
- Modify: `crates/alchimera_core/src/seed.rs`
- Test: `crates/alchimera_core/tests/seed_tests.rs`

**RED:**
- `same_inputs_derive_same_child_seed`
- `different_labels_derive_different_child_seeds`
- `different_indices_derive_different_child_seeds`

**Commands:**
```bash
cargo test -p alchimera_core seed -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- No global RNG.
- Child seed API accepts stable labels/coordinates/indices.
- Tests prove determinism.

**Commit:** `feat: add deterministic world seed derivation`

---

### W1-T2: ChunkCoord Math

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t2-chunk-coord`  
**Branch:** `feat/chunk-coordinates`

**Objective:** Add invisible chunk coordinate conversion for 64m chunks and negative positions.

**Files:**
- Create/Modify: `crates/alchimera_generation/src/chunk.rs` or `crates/alchimera_core/src/chunk.rs` if core-owned
- Test: `crates/alchimera_generation/tests/chunk_coord_tests.rs`

**RED:**
- `origin_maps_to_origin_chunk`
- `positive_world_position_maps_to_expected_chunk`
- `negative_world_position_floors_to_negative_chunk`
- `chunk_world_bounds_are_64m_square`

**Commands:**
```bash
cargo test -p alchimera_generation chunk_coord -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- Correct floor behavior for negative coordinates.
- Constants or settings support default 64m chunk size.

**Commit:** `feat: add chunk coordinate math`

---

### W1-T3: Stable ID Newtypes

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t3-ids`  
**Branch:** `feat/stable-id-types`

**Objective:** Add typed IDs for objects, prototypes, materials, items, recipes, and chunks.

**Files:**
- Modify: `crates/alchimera_core/src/ids.rs`
- Test: `crates/alchimera_core/tests/id_tests.rs`

**RED:**
- `prototype_id_rejects_empty_string`
- `material_id_preserves_namespaced_value`
- `object_id_is_stable_from_seed_chunk_and_index`

**Commands:**
```bash
cargo test -p alchimera_core id -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- ID types are strongly typed, comparable, hashable, serializable if serde is present.
- Object IDs do not depend on Bevy entity IDs or spawn order.

**Commit:** `feat: add stable id newtypes`

---

### W1-T4: Material and Item Definitions

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t4-material-items`  
**Branch:** `feat/material-item-definitions`

**Objective:** Define pure data structures for materials, material properties, item definitions, and alchemical traits.

**Files:**
- Create/Modify: `crates/alchimera_core/src/material.rs`
- Create/Modify: `crates/alchimera_core/src/item.rs`
- Modify: `crates/alchimera_core/src/lib.rs`
- Test: `crates/alchimera_core/tests/material_item_tests.rs`

**RED:**
- `material_definition_rejects_negative_hardness`
- `material_can_store_alchemy_traits`
- `item_definition_references_material_class`

**Commands:**
```bash
cargo test -p alchimera_core material -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- Definitions support hardness, density, flammability, and traits from `SPEC.md`.
- Validation catches invalid numeric ranges.

**Commit:** `feat: add material and item definitions`

---

### W1-T5: Inventory Stack Logic

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t5-inventory`  
**Branch:** `feat/inventory-stacking`

**Objective:** Implement pure inventory slot/stack behavior.

**Files:**
- Modify: `crates/alchimera_core/src/inventory.rs`
- Test: `crates/alchimera_core/tests/inventory_tests.rs`

**RED:**
- `adding_same_item_merges_until_stack_limit`
- `adding_overflow_creates_new_stack`
- `removing_items_decrements_stack`
- `removing_too_many_returns_error`

**Commands:**
```bash
cargo test -p alchimera_core inventory -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- No Bevy dependency.
- Supports fixed-size inventory and stack limits.

**Commit:** `feat: add inventory stack logic`

---

### W1-T6: Recipe Matching Logic

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t6-recipes`  
**Branch:** `feat/recipe-matching`

**Objective:** Implement pure recipe matching for item IDs and material classes.

**Files:**
- Modify: `crates/alchimera_core/src/crafting.rs`
- Test: `crates/alchimera_core/tests/crafting_tests.rs`

**RED:**
- `recipe_matches_exact_item_inputs`
- `recipe_matches_material_class_inputs`
- `recipe_rejects_missing_required_input`
- `crafting_consumes_inputs_and_returns_output_plan`

**Commands:**
```bash
cargo test -p alchimera_core crafting -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- Supports `stone_axe` style recipe shape from `SPEC.md`.
- Produces a pure craft result without Bevy ECS.

**Commit:** `feat: add recipe matching logic`

---

### W1-T7: Save Data Version Shell

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t7-save-shell`  
**Branch:** `feat/save-data-versioning`

**Objective:** Add serializable top-level save data structures with versioning but minimal fields.

**Files:**
- Modify: `crates/alchimera_core/src/save.rs`
- Test: `crates/alchimera_core/tests/save_tests.rs`

**RED:**
- `new_save_data_has_current_version`
- `save_data_roundtrips_through_ron_or_json`
- `unsupported_version_returns_error`

**Commands:**
```bash
cargo test -p alchimera_core save -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- Save schema has explicit version.
- Round-trip tests exist.
- Does not attempt full game persistence yet.

**Commit:** `feat: add save data versioning shell`

---

### W1-T8: Core Error and Validation Helpers

**Worktree:** `/home/openclaw/worktrees/alchimera/w1-t8-validation`  
**Branch:** `feat/core-validation-errors`

**Objective:** Add shared error types and validation helpers for definitions.

**Files:**
- Modify: `crates/alchimera_core/src/error.rs`
- Create: `crates/alchimera_core/src/validation.rs`
- Test: `crates/alchimera_core/tests/validation_tests.rs`

**RED:**
- `duplicate_ids_are_rejected`
- `missing_reference_is_reported_with_context`
- `numeric_range_validation_reports_field_name`

**Commands:**
```bash
cargo test -p alchimera_core validation -- --nocapture
cargo test --workspace
```

**Acceptance criteria:**
- Helpful errors for invalid data files.
- No dependency on Bevy.

**Commit:** `feat: add core validation errors`

---

## 5. Wave 2 Tasks: Generation Foundations

> Start after W1-T1, W1-T2, and W1-T3 are merged. Some tasks additionally depend on W1-T4/W1-T8.

### W2-T1: Biome Selection

**Dependencies:** W1-T1, W1-T2  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t1-biomes`  
**Branch:** `feat/biome-selection`

**Objective:** Implement deterministic biome sampling for initial biomes.

**Files:**
- Modify: `crates/alchimera_generation/src/biome.rs`
- Test: `crates/alchimera_generation/tests/biome_tests.rs`

**RED:**
- `same_seed_position_returns_same_biome`
- `biome_selection_only_returns_initial_biomes`
- `river_valley_can_be_selected_near_low_noise_band`

**Performance:** Add or extend `benches/biome_sampling.rs` if sampling is non-trivial.

**Commit:** `feat: add deterministic biome selection`

---

### W2-T2: Height Sampling

**Dependencies:** W1-T1, W1-T2  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t2-height`  
**Branch:** `feat/height-sampling`

**Objective:** Implement deterministic height sampling for prototype terrain.

**Files:**
- Modify: `crates/alchimera_generation/src/terrain.rs`
- Test: `crates/alchimera_generation/tests/height_sampling_tests.rs`
- Benchmark: `benches/height_sampling.rs`

**RED:**
- `same_seed_same_position_same_height`
- `different_seed_changes_height_summary`
- `height_values_stay_within_configured_bounds`

**Performance command:**
```bash
cargo bench --bench height_sampling
```

**Commit:** `feat: add deterministic height sampling`

---

### W2-T3: Terrain Mesh Data Generation

**Dependencies:** W2-T2  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t3-terrain-mesh`  
**Branch:** `feat/terrain-mesh-data`

**Objective:** Convert height samples into pure mesh data: vertices, normals, UVs, indices, bounds.

**Files:**
- Modify: `crates/alchimera_generation/src/mesh.rs`
- Modify: `crates/alchimera_generation/src/terrain.rs`
- Test: `crates/alchimera_generation/tests/terrain_mesh_tests.rs`
- Benchmark: `benches/terrain_mesh.rs`

**RED:**
- `flat_heightmap_generates_expected_vertex_and_index_count`
- `mesh_bounds_match_chunk_size`
- `normals_are_present_for_each_vertex`

**Performance command:**
```bash
cargo bench --bench terrain_mesh
```

**Commit:** `feat: add terrain mesh data generation`

---

### W2-T4: Object Prototype Schema

**Dependencies:** W1-T3, W1-T4, W1-T8  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t4-prototypes`  
**Branch:** `feat/object-prototype-schema`

**Objective:** Add object prototype definitions and validation.

**Files:**
- Modify: `crates/alchimera_generation/src/objects.rs`
- Test: `crates/alchimera_generation/tests/object_prototype_tests.rs`
- Create: `assets/data/objects/examples/oak_tree.ron`
- Create: `assets/data/objects/examples/granite_boulder.ron`

**RED:**
- `prototype_rejects_missing_generator`
- `prototype_rejects_missing_material_reference`
- `oak_tree_example_loads_and_validates`

**Commit:** `feat: add object prototype schema`

---

### W2-T5: Deterministic Object Placement

**Dependencies:** W1-T1, W1-T2, W1-T3, W2-T1, W2-T2  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t5-object-placement`  
**Branch:** `feat/object-placement`

**Objective:** Generate deterministic object instance placements per chunk.

**Files:**
- Modify: `crates/alchimera_generation/src/objects.rs`
- Test: `crates/alchimera_generation/tests/object_placement_tests.rs`
- Benchmark: `benches/object_placement.rs`

**RED:**
- `same_seed_and_chunk_generate_same_object_instances`
- `different_seed_changes_object_summary`
- `objects_are_within_chunk_bounds`
- `objects_do_not_spawn_on_invalid_slope_when_slope_filter_enabled`

**Performance command:**
```bash
cargo bench --bench object_placement
```

**Commit:** `feat: add deterministic object placement`

---

### W2-T6: Tree Generator Data

**Dependencies:** W1-T1, W1-T3, W2-T4  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t6-tree-generator`  
**Branch:** `feat/tree-generator-data`

**Objective:** Generate pure tree assembly data: trunk segments, branches, leaf clusters, attachment points.

**Files:**
- Create: `crates/alchimera_generation/src/tree.rs`
- Modify: `crates/alchimera_generation/src/lib.rs`
- Test: `crates/alchimera_generation/tests/tree_generator_tests.rs`
- Benchmark: `benches/tree_generator.rs`

**RED:**
- `same_tree_seed_generates_same_summary`
- `tree_has_trunk_and_leaf_clusters`
- `tree_attachment_points_are_stable`

**Performance command:**
```bash
cargo bench --bench tree_generator
```

**Commit:** `feat: add procedural tree generator data`

---

### W2-T7: Rock Generator Data

**Dependencies:** W1-T1, W1-T3, W2-T4  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t7-rock-generator`  
**Branch:** `feat/rock-generator-data`

**Objective:** Generate pure irregular rock/boulder mesh-source data and harvest points.

**Files:**
- Create: `crates/alchimera_generation/src/rock.rs`
- Modify: `crates/alchimera_generation/src/lib.rs`
- Test: `crates/alchimera_generation/tests/rock_generator_tests.rs`
- Benchmark: `benches/rock_generator.rs`

**RED:**
- `same_rock_seed_generates_same_summary`
- `rock_has_nonzero_bounds`
- `rock_harvest_points_are_within_bounds`

**Performance command:**
```bash
cargo bench --bench rock_generator
```

**Commit:** `feat: add procedural rock generator data`

---

### W2-T8: Generation Benchmark Suite

**Dependencies:** W2-T1 through W2-T7 as available  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t8-generation-benches`  
**Branch:** `perf/generation-benchmark-suite`

**Objective:** Add consolidated generation benchmarks and first performance report.

**Files:**
- Create/Modify: `benches/chunk_generation.rs`
- Create: `docs/profiling/YYYY-MM-DD-generation-baseline.md`

**TDD:** Not behavior TDD; verify benchmark compilation and execution.

**Commands:**
```bash
cargo bench --bench chunk_generation
cargo test --workspace
```

**Acceptance criteria:**
- Benchmarks cover biome, height, terrain mesh, object placement for one chunk.
- Report records baseline numbers.

**Commit:** `perf: add generation benchmark baseline`

---

### W2-T9: Chunk Modification Log

**Dependencies:** W1-T2, W1-T3, W1-T7  
**Worktree:** `/home/openclaw/worktrees/alchimera/w2-t9-mod-log`  
**Branch:** `feat/chunk-modification-log`

**Objective:** Add pure serializable chunk modification log for removed/damaged/placed/transformed objects.

**Files:**
- Modify: `crates/alchimera_generation/src/modification_log.rs`
- Test: `crates/alchimera_generation/tests/modification_log_tests.rs`

**RED:**
- `removed_object_override_hides_generated_object`
- `placed_object_is_returned_after_applying_log`
- `modification_log_roundtrips_through_save_format`

**Commit:** `feat: add chunk modification log`

---

## 6. Wave 3 Tasks: Bevy Runtime Foundations

> These tasks can begin after W0 and relevant W1 basics. Keep app registration changes coordinated through small integration merges.

### W3-T1: Bevy App State Shell

**Dependencies:** W0-T1, W0-T3  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t1-app-states`  
**Branch:** `feat/bevy-app-states`

**Objective:** Add app states and root game plugin without gameplay behavior.

**Files:**
- Modify: `crates/alchimera_game/src/lib.rs`
- Modify: `crates/alchimera_game/src/states.rs`
- Modify: `src/main.rs`
- Test: `crates/alchimera_game/tests/app_state_tests.rs`

**RED:**
- `app_registers_expected_states`
- `app_can_enter_in_game_state`

**Commands:**
```bash
cargo test -p alchimera_game app_state -- --nocapture
cargo test --workspace
```

**Commit:** `feat: add bevy app states`

---

### W3-T2: Diagnostics Overlay Resource and Plugin

**Dependencies:** W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t2-diagnostics`  
**Branch:** `feat/diagnostics-overlay`

**Objective:** Add diagnostics plugin and resources for FPS/entity/chunk queue metrics.

**Files:**
- Modify: `crates/alchimera_game/src/diagnostics.rs`
- Test: `crates/alchimera_game/tests/diagnostics_tests.rs`

**RED:**
- `diagnostics_plugin_inserts_metrics_resource`
- `toggle_event_changes_overlay_visibility`

**Performance:** No benchmark; ensure no expensive per-frame scan is introduced.

**Commit:** `feat: add diagnostics overlay skeleton`

---

### W3-T3: Player Controller Skeleton

**Dependencies:** W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t3-player-controller`  
**Branch:** `feat/player-controller-skeleton`

**Objective:** Add player marker/stats/components and movement-intent system without final physics tuning.

**Files:**
- Modify: `crates/alchimera_game/src/player.rs`
- Test: `crates/alchimera_game/tests/player_tests.rs`

**RED:**
- `spawn_player_adds_required_components`
- `movement_input_updates_intent_resource_or_component`

**Commit:** `feat: add player controller skeleton`

---

### W3-T4: Camera and Input Actions

**Dependencies:** W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t4-input-camera`  
**Branch:** `feat/input-camera-actions`

**Objective:** Add input action definitions and camera component setup.

**Files:**
- Create/Modify: `crates/alchimera_game/src/input.rs`
- Modify: `crates/alchimera_game/src/player.rs`
- Test: `crates/alchimera_game/tests/input_camera_tests.rs`

**RED:**
- `default_input_map_contains_move_jump_interact_hotbar`
- `spawn_player_camera_creates_camera_child_or_companion`

**Commit:** `feat: add input and camera actions`

---

### W3-T5: Terrain Rendering Adapter

**Dependencies:** W2-T3, W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t5-terrain-rendering`  
**Branch:** `feat/terrain-rendering-adapter`

**Objective:** Convert pure terrain mesh data into Bevy `Mesh` assets and spawn terrain entities.

**Files:**
- Modify: `crates/alchimera_game/src/world.rs`
- Create: `crates/alchimera_game/src/terrain_rendering.rs`
- Test: `crates/alchimera_game/tests/terrain_rendering_tests.rs`

**RED:**
- `terrain_mesh_data_converts_to_bevy_mesh_with_same_vertex_count`
- `spawn_terrain_chunk_adds_chunk_marker_component`

**Performance:** Measure a simple conversion benchmark if conversion allocates heavily.

**Commit:** `feat: add terrain rendering adapter`

---

### W3-T6: Object Spawning Adapter

**Dependencies:** W2-T5, W2-T6 or W2-T7, W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t6-object-spawning`  
**Branch:** `feat/object-spawning-adapter`

**Objective:** Spawn generated object instances as Bevy entities with names, transforms, and marker components.

**Files:**
- Modify: `crates/alchimera_game/src/objects.rs`
- Test: `crates/alchimera_game/tests/object_spawning_tests.rs`

**RED:**
- `spawn_generated_object_adds_world_object_component`
- `spawned_object_preserves_stable_object_id`
- `spawned_object_has_transform_from_generated_instance`

**Commit:** `feat: add object spawning adapter`

---

### W3-T7: Chunk Streaming Resource and Systems

**Dependencies:** W1-T2, W2-T2, W2-T5, W3-T1, W3-T5, W3-T6  
**Worktree:** `/home/openclaw/worktrees/alchimera/w3-t7-chunk-streaming`  
**Branch:** `feat/chunk-streaming-systems`

**Objective:** Request, spawn, and despawn chunks around the player using an active radius.

**Files:**
- Modify: `crates/alchimera_game/src/world.rs`
- Create: `crates/alchimera_game/src/streaming.rs`
- Test: `crates/alchimera_game/tests/chunk_streaming_tests.rs`

**RED:**
- `player_at_origin_requests_origin_and_neighbor_chunks`
- `moving_player_requests_new_chunk`
- `chunks_outside_radius_are_marked_for_despawn`

**Performance:** Add diagnostics counter for active chunks and queue length.

**Commit:** `feat: add chunk streaming systems`

---

## 7. Wave 4 Tasks: Gameplay Vertical Slice

### W4-T1: Interaction Raycast Events

**Dependencies:** W3-T3, W3-T4, W3-T6  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t1-interaction`  
**Branch:** `feat/interaction-raycast-events`

**Objective:** Add interaction targeting event model and testable selected-object state.

**Files:**
- Create: `crates/alchimera_game/src/interaction.rs`
- Modify: `crates/alchimera_game/src/lib.rs`
- Test: `crates/alchimera_game/tests/interaction_tests.rs`

**RED:**
- `raycast_hit_updates_current_interaction_target`
- `no_hit_clears_current_interaction_target`

**Commit:** `feat: add interaction raycast events`

---

### W4-T2: Harvesting ECS System

**Dependencies:** W1-T5, W2-T9, W3-T6, W4-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t2-harvesting`  
**Branch:** `feat/harvesting-system`

**Objective:** Add events and systems for harvesting objects into inventory and modification logs.

**Files:**
- Modify: `crates/alchimera_game/src/objects.rs`
- Create: `crates/alchimera_game/src/harvesting.rs`
- Test: `crates/alchimera_game/tests/harvesting_tests.rs`

**RED:**
- `harvest_event_adds_yield_to_inventory`
- `harvest_event_marks_object_harvested`
- `harvested_generated_object_adds_removed_or_state_override_to_mod_log`

**Commit:** `feat: add harvesting system`

---

### W4-T3: Inventory Hotbar Model and UI Shell

**Dependencies:** W1-T5, W3-T1  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t3-inventory-ui`  
**Branch:** `feat/inventory-hotbar-ui-shell`

**Objective:** Add Bevy resources/components for inventory and hotbar selection plus minimal UI shell.

**Files:**
- Create/Modify: `crates/alchimera_game/src/inventory_ui.rs`
- Modify: `crates/alchimera_game/src/ui.rs`
- Test: `crates/alchimera_game/tests/inventory_ui_tests.rs`

**RED:**
- `hotbar_selection_wraps_or_clamps_as_configured`
- `inventory_resource_initializes_with_fixed_slot_count`

**Commit:** `feat: add inventory hotbar ui shell`

---

### W4-T4: Hand Crafting ECS System

**Dependencies:** W1-T5, W1-T6, W4-T3  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t4-crafting`  
**Branch:** `feat/hand-crafting-system`

**Objective:** Connect pure recipe matching to Bevy events/resources for hand crafting.

**Files:**
- Create: `crates/alchimera_game/src/crafting.rs`
- Test: `crates/alchimera_game/tests/crafting_system_tests.rs`

**RED:**
- `craft_event_consumes_inventory_inputs`
- `craft_event_adds_output_item`
- `craft_event_fails_without_required_inputs`

**Commit:** `feat: add hand crafting system`

---

### W4-T5: Basic Build Placement Command

**Dependencies:** W1-T3, W3-T6, W4-T1, W4-T3  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t5-building`  
**Branch:** `feat/basic-build-placement`

**Objective:** Add placement command for workbench/beam/wall objects using ghost transform data.

**Files:**
- Create: `crates/alchimera_game/src/building.rs`
- Test: `crates/alchimera_game/tests/building_tests.rs`

**RED:**
- `place_object_event_spawns_player_object`
- `placement_consumes_inventory_item`
- `placed_object_receives_stable_player_object_id`

**Commit:** `feat: add basic build placement`

---

### W4-T6: Save/Load Integration

**Dependencies:** W1-T7, W2-T9, W4-T2, W4-T5  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t6-save-load`  
**Branch:** `feat/save-load-integration`

**Objective:** Persist world seed, inventory, chunk modifications, and placed objects.

**Files:**
- Create: `crates/alchimera_game/src/persistence.rs`
- Modify: `crates/alchimera_core/src/save.rs` only if strictly needed
- Test: `crates/alchimera_game/tests/persistence_tests.rs`

**RED:**
- `saving_and_loading_preserves_inventory`
- `saving_and_loading_preserves_harvested_object_override`
- `saving_and_loading_preserves_placed_object`

**Performance:** Add broad save/load timing test or benchmark if save grows beyond trivial.

**Commit:** `feat: add save load integration`

---

### W4-T7: Alchemy Trait Core Slice

**Dependencies:** W1-T4, W1-T5, W1-T6  
**Worktree:** `/home/openclaw/worktrees/alchimera/w4-t7-alchemy`  
**Branch:** `feat/alchemy-trait-core`

**Objective:** Add pure alchemy trait discovery and two-ingredient experiment logic.

**Files:**
- Create: `crates/alchimera_core/src/alchemy.rs`
- Modify: `crates/alchimera_core/src/lib.rs`
- Test: `crates/alchimera_core/tests/alchemy_tests.rs`

**RED:**
- `inspecting_material_discovers_trait`
- `combining_compatible_traits_produces_reagent`
- `unknown_combination_returns_failed_experiment_result`

**Commit:** `feat: add alchemy trait core`

---

## 8. Wave 5 Tasks: Integration, Profiling, Playtest

### W5-T1: Vertical Slice Integration Branch

**Parallel safety:** Serial integration task.

**Branch:** `integration/vertical-slice`

**Objective:** Merge Wave 4 features and ensure the game can boot into a world with terrain, objects, harvesting, inventory, crafting, placement, and save/load.

**Commands:**
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --release
cargo bench --workspace -- --profile-time 5
cargo run
```

**Acceptance criteria:**
- All automated checks pass.
- Game boots.
- Manual playtest checklist can be attempted.

**Commit/Merge:** merge commits allowed for integration branch.

---

### W5-T2: Manual Playtest Checklist for MVP Slice

**Parallel safety:** Can run after W5-T1 build is available.

**Worktree:** `/home/openclaw/worktrees/alchimera/w5-t2-playtest`  
**Branch:** `docs/mvp-playtest-checklist`

**Objective:** Create and run a manual playtest checklist for the MVP vertical slice.

**Files:**
- Create: `docs/playtests/YYYY-MM-DD-mvp-vertical-slice.md`

**Checklist must include:**
- Boot game.
- Start world with seed.
- Walk on terrain.
- Observe terrain chunk loading.
- See trees/rocks.
- Target object.
- Harvest wood/stone.
- Open inventory/hotbar.
- Craft stone axe or placeholder recipe.
- Place workbench/beam/wall.
- Save, quit, reload.
- Confirm inventory and world modifications persist.
- Record FPS/frame-time notes.

**Commit:** `docs: add mvp vertical slice playtest`

---

### W5-T3: Performance Baseline Report

**Parallel safety:** Can run after W5-T1 build is available.

**Worktree:** `/home/openclaw/worktrees/alchimera/w5-t3-perf-baseline`  
**Branch:** `perf/mvp-baseline-report`

**Objective:** Record initial performance baseline for generation and runtime.

**Files:**
- Create: `docs/profiling/YYYY-MM-DD-mvp-baseline.md`

**Commands:**
```bash
cargo bench --workspace
cargo run --release
```

**Report:**
- Generation benchmark means.
- Active chunk radius tested.
- FPS/frame time observed.
- Entity count observed.
- Known bottlenecks.
- Next perf tasks.

**Commit:** `perf: record mvp baseline performance`

---

### W5-T4: Final Integration Review

**Parallel safety:** Reviewer only; no code changes unless explicitly requested.

**Worktree:** `/home/openclaw/worktrees/alchimera/w5-t4-final-review`  
**Branch:** `review/mvp-vertical-slice`

**Objective:** Perform whole-project review against `SPEC.md`, `IMPLEMENTATION_GUIDELINES.md`, and this plan.

**Review checks:**
- Does MVP satisfy required vertical slice?
- Are tests meaningful and TDD-aligned?
- Are deterministic generation guarantees tested?
- Are performance measurements present?
- Are ECS systems modular?
- Are save/load tests present?
- Are worktrees/branches clean?

**Output:** `docs/reviews/YYYY-MM-DD-mvp-final-review.md`

**Commit:** `docs: add mvp final integration review`

---

## 9. Maximum Parallel Execution Schedule

### Batch A: Bootstrap

Run serial:

1. W0-T1

Then run in parallel:

- W0-T2
- W0-T3
- W0-T4
- W0-T5

### Batch B: Core Foundations

After W0 merged, run in parallel:

- W1-T1 seed
- W1-T2 chunk coord
- W1-T3 IDs
- W1-T4 material/item definitions
- W1-T5 inventory
- W1-T6 recipe logic
- W1-T7 save shell
- W1-T8 validation

Expected max parallel implementers: 8, if available.

### Batch C: Generation Foundations Round 1

After W1-T1/T2/T3 merged, run in parallel:

- W2-T1 biome selection
- W2-T2 height sampling
- W2-T4 object prototype schema, if W1-T4/T8 are also merged
- W2-T9 modification log, if W1-T7 is merged

Expected max parallel implementers: 4.

### Batch D: Generation Foundations Round 2

After Batch C dependencies, run in parallel:

- W2-T3 terrain mesh
- W2-T5 object placement
- W2-T6 tree generator
- W2-T7 rock generator
- W2-T8 generation benchmark suite, once enough benchmarks exist

Expected max parallel implementers: 4-5.

### Batch E: Bevy Runtime Round 1

After W0 and W1 basics, run:

- W3-T1 app states first, serial.

Then run in parallel:

- W3-T2 diagnostics
- W3-T3 player controller
- W3-T4 input/camera

Expected max parallel implementers after W3-T1: 3.

### Batch F: Bevy Runtime Round 2

After W2 mesh/object work and W3 app shell:

- W3-T5 terrain rendering
- W3-T6 object spawning

Then serial/near-serial:

- W3-T7 chunk streaming

Expected max parallel implementers: 2, then 1.

### Batch G: Gameplay Round 1

After W3 runtime and W1 gameplay logic:

- W4-T1 interaction
- W4-T3 inventory UI shell
- W4-T7 alchemy core

Expected max parallel implementers: 3.

### Batch H: Gameplay Round 2

After Batch G dependencies:

- W4-T2 harvesting
- W4-T4 crafting system
- W4-T5 building placement

Expected max parallel implementers: 3.

### Batch I: Persistence and Integration

Run:

- W4-T6 save/load integration
- W5-T1 vertical slice integration

Then parallel docs/review:

- W5-T2 playtest checklist
- W5-T3 performance baseline
- W5-T4 final review

Expected max parallel implementers: 1, then 3.

---

## 10. Subagent Prompt Template for Any Task

Use this template when dispatching implementation subagents:

```text
Project: Alchimera
Language: Rust
Engine: Bevy
Repository/worktree: [absolute worktree path]
Branch: [branch]

Relevant docs:
- /home/openclaw/alchimera/SPEC.md
- /home/openclaw/alchimera/IMPLEMENTATION_GUIDELINES.md
- /home/openclaw/alchimera/docs/plans/parallel-subagent-task-breakdown.md

Task:
[paste exact task section]

Hard rules:
1. Work only in the assigned worktree.
2. Do not modify files outside the task's allowed file list unless blocked; if blocked, report instead of expanding scope.
3. Follow strict TDD: failing test first, observe failure, implement minimal code, observe pass.
4. Run required commands.
5. Run benchmarks if the task declares performance requirements.
6. Commit the completed task.
7. Report files changed, tests added, commands run, results, benchmark numbers if any, commit hash, and unresolved issues.
```

---

## 11. Reviewer Prompt Template

```text
Project: Alchimera
Repository/worktree: [absolute path]
Branch/commit under review: [branch or sha]

Review against:
- SPEC.md
- IMPLEMENTATION_GUIDELINES.md
- docs/plans/parallel-subagent-task-breakdown.md

Original task:
[paste exact task]

Review type:
[Spec compliance OR Quality/performance]

Required output:
Verdict: PASS/APPROVED/REQUEST_CHANGES
Critical issues:
Important issues:
Minor issues:
Commands run:
Required fixes:
```

---

## 12. Orchestrator Merge Checklist

Before merging any task branch:

- [ ] Implementation subagent reported commit hash.
- [ ] Spec reviewer returned `PASS`.
- [ ] Quality/performance reviewer returned `APPROVED`.
- [ ] Orchestrator inspected `git diff --stat`.
- [ ] Orchestrator ran or verified required tests.
- [ ] Performance report exists if required.
- [ ] Branch touches only intended files or deviations are justified.
- [ ] Worktree is clean.

Before merging a wave:

- [ ] All task branches in the wave are merged into integration branch.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo test --workspace` passes.
- [ ] Relevant benchmarks pass.
- [ ] Dependency graph is updated if tasks changed.

---

## 13. First Recommended Subagent Dispatches

Once W0-T1 is complete, dispatch these four immediately:

1. W0-T2 developer README commands.
2. W0-T3 module skeletons.
3. W0-T4 benchmark harness.
4. W0-T5 asset data layout.

Once Wave 0 is merged, dispatch all Wave 1 tasks in parallel.

This gives the fastest route to broad parallel development while keeping merge conflicts manageable.
