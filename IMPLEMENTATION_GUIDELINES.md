# Alchimera Implementation Guidelines

> Companion document to `SPEC.md`. This file defines *how* Alchimera should be implemented: strict TDD, measurable performance, isolated worktrees, and subagent-driven development.

**Project:** Alchimera  
**Engine:** Bevy  
**Language:** Rust  
**Primary goal:** Build a high-confidence, performance-conscious 3D procedural open-world game without letting complexity outrun tests, profiling, and code review.

---

## 1. Guiding Principles

### 1.1 Test First

No gameplay logic, world generation logic, inventory behavior, crafting rule, save/load behavior, or deterministic algorithm should be implemented before a failing test exists.

For Alchimera, TDD is not bureaucracy. It is protection against procedural-game chaos.

Procedural systems are especially prone to:

- Hidden nondeterminism.
- Seed/version regressions.
- Performance cliffs.
- Save/load corruption.
- ECS order-of-execution bugs.
- Behavior that only fails after many chunks or objects.

The required loop is always:

1. **RED:** Write a failing test for one behavior.
2. **VERIFY RED:** Run the test and confirm it fails for the expected reason.
3. **GREEN:** Write the smallest implementation that passes.
4. **VERIFY GREEN:** Run the test and confirm it passes.
5. **REGRESSION CHECK:** Run the relevant broader test suite.
6. **REFACTOR:** Clean up only while tests remain green.
7. **MEASURE:** If the change affects runtime, memory, generation, rendering, physics, or asset loading, run the relevant benchmark/profile.
8. **COMMIT:** Commit a small, coherent change.

### 1.2 Measure Performance Continuously

Alchimera is a 3D open-world Bevy game. Correctness is not enough. A correct chunk generator that stalls the frame for 200 ms is a bug.

Performance must be measured from the beginning for:

- Chunk generation time.
- Mesh generation time.
- Collider generation time.
- Object placement time.
- Active entity count.
- Frame time.
- Physics step time.
- Memory usage.
- Asset loading time.
- Save/load time.

Every system that can scale with world size needs a budget and a measurement.

### 1.3 Isolate Work with Git Worktrees

Each feature, experiment, bug fix, or subagent task should happen in its own branch and preferably its own worktree.

This prevents:

- Subagents overwriting each other.
- Half-finished experiments polluting `main`.
- Review confusion.
- Generated artifacts accidentally bleeding into other work.

### 1.4 Use Subagents for Focused, Reviewable Tasks

Subagents should implement small tasks in isolated contexts, then separate reviewer subagents should verify:

1. Spec compliance.
2. Code quality.
3. Tests and performance impact.

Do not let one subagent implement a large milestone alone without review gates.

### 1.5 Keep Systems Data-Oriented and Modular

Bevy ECS should be used intentionally.

Prefer:

- Components as data.
- Systems as behavior.
- Events/commands for cross-system changes.
- Resources for shared configuration and registries.
- Plugins for feature modules.
- Pure functions for deterministic generation and tests.

Avoid:

- Large god resources.
- Hidden global mutable state.
- Gameplay logic embedded directly in rendering systems.
- Tests that require launching the full game window unless unavoidable.

---

## 2. Repository and Branch Workflow

### 2.1 Default Repository Layout

Until implementation begins, use this target layout:

```text
alchimera/
  Cargo.toml
  Cargo.lock
  SPEC.md
  IMPLEMENTATION_GUIDELINES.md
  README.md
  assets/
    data/
      materials/
      objects/
      recipes/
      biomes/
      alchemy/
  benches/
    chunk_generation.rs
    terrain_mesh.rs
    object_placement.rs
  crates/
    alchimera_core/
      src/
      tests/
    alchimera_game/
      src/
    alchimera_generation/
      src/
      tests/
  docs/
    decisions/
    plans/
    profiling/
  src/
    main.rs
  tests/
```

Recommended crate split once complexity justifies it:

- `alchimera_core`: IDs, math helpers, deterministic seed utilities, inventory/crafting core, save data types.
- `alchimera_generation`: terrain, biome, chunk, object placement, procedural mesh source data.
- `alchimera_game`: Bevy app, plugins, systems, rendering, physics integration.
- Root binary: launches the game.

Do not over-split on day one. Start simple, then extract crates when tests and compile times benefit.

### 2.2 Main Branch Rules

The `main` branch must always be:

- Buildable.
- Testable.
- Free of known broken gameplay logic.
- Free of unreviewed large experiments.

Before merging into `main`, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --release
cargo bench --workspace -- --profile-time 5
```

If benchmarks are too expensive for every merge early in development, at minimum run the targeted benchmarks for affected systems.

### 2.3 Branch Naming

Use descriptive branch names:

```text
feat/bootstrap-bevy-app
feat/chunk-coordinates
feat/terrain-heightmap-v1
feat/object-prototype-registry
feat/tree-generator-v1
feat/inventory-stacking
fix/chunk-seed-nondeterminism
perf/object-placement-cache
spike/sdf-terrain-prototype
```

Prefix meanings:

- `feat/`: production feature.
- `fix/`: bug fix with regression test.
- `perf/`: performance improvement with measurement before/after.
- `refactor/`: internal change preserving behavior.
- `docs/`: documentation only.
- `spike/`: throwaway exploration; not merged unless rewritten with tests.

---

## 3. Git Worktree Workflow

### 3.1 Why Worktrees

Worktrees allow multiple branches to be checked out at the same time in separate directories.

Use them for:

- Parallel subagent work.
- Risky experiments.
- Benchmarking a branch while another task continues.
- Review branches.
- Comparing before/after performance.

### 3.2 Recommended Directory Structure

From the main repository at:

```bash
/home/openclaw/alchimera
```

Create sibling worktrees under:

```bash
/home/openclaw/worktrees/alchimera/
```

Example:

```text
/home/openclaw/alchimera                         # main checkout
/home/openclaw/worktrees/alchimera/bootstrap     # feat/bootstrap-bevy-app
/home/openclaw/worktrees/alchimera/chunks        # feat/chunk-coordinates
/home/openclaw/worktrees/alchimera/terrain       # feat/terrain-heightmap-v1
/home/openclaw/worktrees/alchimera/review        # reviewer checkout
```

### 3.3 Create a Worktree

From `/home/openclaw/alchimera`:

```bash
mkdir -p /home/openclaw/worktrees/alchimera

git worktree add \
  -b feat/chunk-coordinates \
  /home/openclaw/worktrees/alchimera/chunk-coordinates
```

Then work inside:

```bash
cd /home/openclaw/worktrees/alchimera/chunk-coordinates
```

### 3.4 List Worktrees

```bash
git worktree list
```

### 3.5 Remove a Finished Worktree

After the branch is merged or no longer needed:

```bash
git worktree remove /home/openclaw/worktrees/alchimera/chunk-coordinates
```

If files are dirty, inspect first:

```bash
git -C /home/openclaw/worktrees/alchimera/chunk-coordinates status --short
```

Only force-remove when the dirty state is intentionally disposable:

```bash
git worktree remove --force /home/openclaw/worktrees/alchimera/chunk-coordinates
```

### 3.6 Worktree Safety Rules for Subagents

Every implementation subagent must receive:

- Exact worktree path.
- Exact branch name.
- Task scope.
- Files it may modify.
- Test commands it must run.
- Benchmark commands it must run if performance-sensitive.
- Commit message format.

Subagents must not work directly in `/home/openclaw/alchimera` unless explicitly assigned.

Subagents must not touch another subagent's worktree.

Subagents must not merge branches.

Only the orchestrator should:

- Merge branches.
- Resolve cross-task conflicts.
- Delete worktrees.
- Decide whether benchmark regressions are acceptable.

---

## 4. TDD Standard for Rust and Bevy

### 4.1 What Must Be Tested

Test every pure or mostly pure system:

- Seed hashing.
- Chunk coordinate conversion.
- Chunk bounds.
- Biome selection.
- Terrain height sampling.
- Resource/object placement determinism.
- Object ID generation.
- Object prototype parsing.
- Inventory stacking.
- Recipe matching.
- Tool durability.
- Harvest result calculation.
- Save/load serialization.
- Modification log application.
- Alchemy trait discovery.
- Building snap-point rules.

Use Bevy app/world tests for ECS behavior:

- Systems add/remove expected components.
- Events produce expected world mutations.
- State transitions happen correctly.
- Harvest events update inventory and object state.
- Chunk streaming spawns/despawns expected entities.

Use manual playtests only for feel, visuals, and final integration. Manual playtests do not replace automated tests.

### 4.2 Rust Test Locations

Use unit tests near pure logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_coord_from_world_position_handles_negative_coordinates() {
        let coord = ChunkCoord::from_world_xz(-0.1, -64.1, 64.0);
        assert_eq!(coord, ChunkCoord { x: -1, z: -2 });
    }
}
```

Use integration tests for cross-module behavior:

```text
crates/alchimera_generation/tests/chunk_generation_tests.rs
crates/alchimera_core/tests/inventory_tests.rs
tests/save_load_roundtrip.rs
```

Use Bevy ECS tests when behavior depends on schedules, resources, events, or components:

```rust
#[test]
fn harvest_event_adds_resource_to_inventory() {
    let mut app = App::new();
    app.add_event::<HarvestEvent>();
    app.add_systems(Update, apply_harvest_events);
    app.insert_resource(Inventory::default());

    app.world_mut().send_event(HarvestEvent {
        resource: ItemId::new("wood.oak.log"),
        amount: 1,
    });

    app.update();

    let inventory = app.world().resource::<Inventory>();
    assert_eq!(inventory.count(ItemId::new("wood.oak.log")), 1);
}
```

### 4.3 TDD Cycle Commands

For a single test:

```bash
cargo test chunk_coord_from_world_position_handles_negative_coordinates --workspace -- --nocapture
```

For one crate:

```bash
cargo test -p alchimera_generation
```

For all tests:

```bash
cargo test --workspace
```

For release-mode tests when behavior may differ due to optimization or numeric assumptions:

```bash
cargo test --workspace --release
```

For doc tests:

```bash
cargo test --workspace --doc
```

### 4.4 RED Requirements

When writing a failing test, confirm:

- The test compiles or fails only because the target behavior/API is missing.
- The failure is not caused by a typo, syntax error, or incorrect import.
- The failure message proves the test is meaningful.
- The test name describes behavior, not implementation.

Good test name:

```text
same_seed_generates_same_tree_positions_in_chunk
```

Bad test name:

```text
test_tree_gen
```

### 4.5 GREEN Requirements

When making the test pass:

- Implement only the behavior required by the failing test.
- Do not opportunistically add adjacent systems.
- Do not refactor unrelated code.
- Do not add untested branches.
- Do not tune performance before correctness unless the test itself is a performance regression test.

### 4.6 REFACTOR Requirements

After tests are green:

- Run the same test again.
- Run the relevant module/crate suite.
- Then refactor.
- Re-run tests after each meaningful refactor.

Refactor commits should not change behavior. If behavior changes, add a failing test first and use a feature/fix commit instead.

### 4.7 Tests for Determinism

Procedural generation must be deterministic.

Minimum determinism tests:

```rust
#[test]
fn same_seed_and_chunk_generate_same_object_instances() {
    let seed = WorldSeed::new(12345);
    let coord = ChunkCoord { x: 4, z: -2 };

    let a = generate_chunk_objects(seed, coord);
    let b = generate_chunk_objects(seed, coord);

    assert_eq!(a, b);
}

#[test]
fn different_seeds_generate_different_object_instances() {
    let coord = ChunkCoord { x: 0, z: 0 };

    let a = generate_chunk_objects(WorldSeed::new(1), coord);
    let b = generate_chunk_objects(WorldSeed::new(2), coord);

    assert_ne!(a, b);
}
```

Avoid using non-seeded global randomness inside generation code. Use explicit seeded RNG passed through context.

### 4.8 Tests for Save/Load

Every persisted system needs round-trip tests.

Example requirements:

- Serialize save data.
- Deserialize save data.
- Apply deserialized modifications to regenerated chunk.
- Confirm harvested/placed/transformed objects match expected state.

The save format should include versioning from the start.

### 4.9 Tests for Floating-Point Behavior

Avoid exact equality for generated floating-point mesh values unless the values are guaranteed integer-derived.

Use approximate comparisons:

```rust
fn assert_close(a: f32, b: f32, epsilon: f32) {
    assert!((a - b).abs() <= epsilon, "{a} != {b} within {epsilon}");
}
```

For deterministic generation, prefer asserting stable IDs, counts, bounds, and hashes over raw vertex-by-vertex equality unless the generator is intentionally locked.

### 4.10 Snapshot and Golden Tests

For procedural outputs, use compact golden summaries rather than giant mesh snapshots.

Good snapshot data:

- Object count by prototype.
- Terrain height min/max/mean.
- Mesh vertex count.
- Mesh index count.
- Stable hash of quantized positions.
- Stable hash of object IDs and transforms.

Avoid committing massive binary outputs as test fixtures.

---

## 5. Performance Measurement Standard

### 5.1 Performance Is a Feature

A change is incomplete if it adds a scalable system without measuring it.

Performance-sensitive systems include:

- World generation.
- Chunk streaming.
- Terrain mesh generation.
- Procedural object generation.
- Collider generation.
- Save/load.
- Inventory/crafting with large item sets.
- ECS queries that run every frame.
- Rendering of repeated objects.
- Physics for many colliders.

### 5.2 Initial Performance Budgets

These are prototype budgets. Revise them as the game matures, but do not leave systems budgetless.

#### Frame Time

- Target: 60 FPS.
- Total frame budget: 16.67 ms.
- Main-thread gameplay/update budget: 4 ms.
- Render preparation budget: 4 ms.
- Physics budget: 2 ms.
- Streaming/background handoff budget: 1 ms per frame on main thread.

#### Chunk Generation

For a `64m x 64m` chunk in prototype quality:

- Terrain height sampling: <= 2 ms per chunk.
- Terrain mesh generation: <= 8 ms per chunk on worker thread.
- Object placement: <= 4 ms per chunk.
- Collider generation: <= 8 ms per chunk on worker thread.
- Main-thread spawn/despawn handoff: <= 2 ms per frame.

#### Entity Counts

Early target limits near player:

- Interactable objects: <= 500 active.
- Decorative/detail objects: <= 5,000 visible or instanced.
- Physics colliders: <= 1,000 active, fewer preferred.
- Dynamic rigid bodies: <= 100 active.

#### Memory

Prototype target:

- Active world/chunk memory: <= 1 GB.
- Avoid unbounded mesh cache growth.
- Cap generated asset caches and evict distant chunks.

### 5.3 Benchmarking Tools

Recommended Rust benchmarking:

- `criterion` for stable micro/macro benchmarks.
- `cargo bench` for benchmark execution.
- `iai-callgrind` later for deterministic instruction-level benchmarking on pure functions.

Recommended profiling:

- `tracing` and `tracing-subscriber` for spans.
- Bevy diagnostics for FPS/frame time/entity count.
- `bevy::diagnostic::FrameTimeDiagnosticsPlugin`.
- `bevy::diagnostic::EntityCountDiagnosticsPlugin`.
- `cargo flamegraph` for CPU profiling.
- Tracy integration later if needed.
- RenderDoc for graphics frame analysis when rendering issues arise.

### 5.4 Add Diagnostics Early

The game should expose a debug overlay with:

- FPS.
- Frame time.
- Entity count.
- Active chunk count.
- Chunk generation queue length.
- Mesh generation queue length.
- Collider generation queue length.
- Loaded procedural mesh count.
- Active physics collider count.
- Player chunk coordinate.

This overlay should be toggled by a debug key and should not require a special build.

### 5.5 Instrument Systems with Spans

Long-running systems should have tracing spans.

Example:

```rust
use tracing::instrument;

#[instrument(skip(context))]
pub fn generate_chunk(context: &ChunkGenerationContext) -> GeneratedChunk {
    // generation logic
}
```

For Bevy systems, use spans around expensive sections:

```rust
fn stream_chunks_system(/* params */) {
    let _span = tracing::info_span!("stream_chunks_system").entered();
    // streaming logic
}
```

### 5.6 Criterion Benchmarks

Create focused benchmarks for pure generation logic.

Example target files:

```text
benches/chunk_generation.rs
benches/terrain_mesh.rs
benches/object_placement.rs
benches/save_load.rs
```

Example benchmark shape:

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_generate_chunk_objects(c: &mut Criterion) {
    c.bench_function("generate_chunk_objects_seed_12345_origin", |b| {
        b.iter(|| {
            let seed = WorldSeed::new(12345);
            let coord = ChunkCoord { x: 0, z: 0 };
            generate_chunk_objects(seed, coord)
        });
    });
}

criterion_group!(benches, bench_generate_chunk_objects);
criterion_main!(benches);
```

Run:

```bash
cargo bench --bench object_placement
```

### 5.7 Before/After Performance Protocol

For any `perf/` branch or any change likely to affect performance:

1. Create baseline from target branch.
2. Run relevant benchmark at least 3 times if noisy.
3. Record baseline in `docs/profiling/YYYY-MM-DD-topic.md`.
4. Implement change with tests.
5. Run same benchmark again.
6. Record after numbers.
7. Explain improvement or regression.
8. Include benchmark output in PR/merge notes.

Template:

```markdown
# Performance Report: [Topic]

Date: YYYY-MM-DD
Branch: perf/[name]
Base commit: [sha]
Change commit: [sha]
Machine: [CPU/GPU/RAM if known]
Build: release

## Scenario

What was measured and why.

## Command

```bash
cargo bench --bench object_placement
```

## Baseline

- Mean:
- Std dev:
- Notes:

## After

- Mean:
- Std dev:
- Notes:

## Result

- Change:
- Accepted? yes/no
- Follow-up:
```

### 5.8 Performance Regression Tests

Some performance checks can be automated as non-flaky threshold tests, especially for pure functions.

Use threshold tests carefully. They should be broad enough to avoid failing from noise but strict enough to catch disasters.

Example:

```rust
#[test]
fn object_placement_for_one_chunk_completes_under_reasonable_time() {
    let start = std::time::Instant::now();
    let _objects = generate_chunk_objects(WorldSeed::new(12345), ChunkCoord { x: 0, z: 0 });
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 100, "object placement took {elapsed:?}");
}
```

Do not use tight millisecond thresholds in normal unit tests. Use benchmarks for precise performance tracking.

### 5.9 Profiling Before Optimizing

Do not optimize based on guesses.

Required optimization workflow:

1. Identify symptom.
2. Reproduce it.
3. Measure baseline.
4. Profile to find hot path.
5. Write correctness tests for intended behavior.
6. Implement optimization.
7. Re-run tests.
8. Re-measure performance.
9. Keep optimization only if it improves the measured target without unacceptable complexity.

### 5.10 Performance Commit Messages

Use commit messages that include the measured result:

```text
perf: cache object placement noise samples

Object placement benchmark, origin chunk:
- before: 5.8 ms mean
- after: 2.1 ms mean
```

---

## 6. Subagent-Driven Development Workflow

### 6.1 When to Use Subagents

Use subagents for:

- Implementing a task from a plan.
- Reviewing spec compliance.
- Reviewing code quality.
- Reviewing performance claims.
- Investigating a bug in isolation.
- Comparing implementation options.
- Writing tests for an existing untested module before refactor.

Do not use subagents for:

- Tasks requiring continuous user interaction.
- Large vague milestones without decomposition.
- Multiple agents editing the same files at the same time.
- Final merge decisions.

### 6.2 Required Roles

For each implementation task, use three distinct roles where practical.

#### Implementer

Responsibilities:

- Work in assigned worktree.
- Follow strict TDD.
- Make a small implementation.
- Run required tests.
- Run required benchmarks if applicable.
- Commit changes.
- Report exact files changed, commands run, and results.

#### Spec Reviewer

Responsibilities:

- Compare implementation against task spec and `SPEC.md`.
- Verify no requirements are missing.
- Verify no scope creep was added.
- Verify file paths and architecture match expectations.
- Output `PASS` or specific required changes.

#### Quality/Performance Reviewer

Responsibilities:

- Review code clarity, maintainability, Bevy ECS usage, Rust idioms, and test quality.
- Check for hidden nondeterminism.
- Check for unnecessary allocations or per-frame work.
- Check benchmark/profiling evidence when relevant.
- Output `APPROVED` or `REQUEST_CHANGES`.

### 6.3 Subagent Task Size

A subagent implementation task should be small enough to finish in one focused pass.

Good task sizes:

- Add `ChunkCoord` and tests.
- Add deterministic seed hashing utility.
- Add inventory stack merge behavior.
- Add object prototype parser for one data format.
- Add criterion benchmark for object placement.
- Add Bevy diagnostic overlay skeleton.

Too large:

- Implement terrain generation.
- Implement crafting.
- Implement all player controls.
- Build multiplayer architecture.
- Make a complete Minecraft clone.

Split large tasks into 2-5 minute implementation slices.

### 6.4 Subagent Context Template

Every implementer subagent should receive context in this structure:

```text
Project: Alchimera
Language: Rust
Engine: Bevy
Repository/worktree: [absolute path]
Branch: [branch name]

Relevant docs:
- SPEC.md sections: [list]
- IMPLEMENTATION_GUIDELINES.md sections: [list]

Task objective:
[one sentence]

Allowed files:
- [paths]

Do not modify:
- [paths]

TDD requirements:
1. Write failing test first.
2. Run exact test and confirm expected failure.
3. Implement minimal code.
4. Run exact test and confirm pass.
5. Run broader relevant suite.
6. Commit.

Commands to run:
- cargo fmt --all
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace
- [targeted benchmark if applicable]

Performance requirements:
- [budget or measurement command]

Commit message:
- [exact format]

Report back:
- Files changed.
- Tests added.
- Commands run and results.
- Benchmarks/profiling if applicable.
- Commit hash.
- Any unresolved issues.
```

### 6.5 Reviewer Context Template

Reviewer subagents should receive:

```text
Project: Alchimera
Repository/worktree: [absolute path]
Branch/commit under review: [branch or sha]

Original task spec:
[paste full task]

Relevant docs:
- SPEC.md sections: [list]
- IMPLEMENTATION_GUIDELINES.md sections: [list]

Review type:
- Spec compliance OR quality/performance

Required checks:
- [checklist]

Output format:
- Verdict: PASS / APPROVED / REQUEST_CHANGES
- Critical issues:
- Important issues:
- Minor issues:
- Required fixes:
- Commands run:
```

### 6.6 Two-Stage Review Gate

Every task must pass gates in order:

1. Implementer completes and commits.
2. Spec reviewer returns `PASS`.
3. Quality/performance reviewer returns `APPROVED`.
4. Orchestrator runs or verifies final commands.
5. Branch is eligible for merge.

Do not run quality review before spec review passes. It wastes effort polishing code that may not satisfy the task.

### 6.7 Handling Review Failures

If spec review fails:

- Send the required fixes to an implementer subagent.
- Keep the same worktree/branch.
- Require tests for missing behavior.
- Re-run spec review.
- Only then run quality review.

If quality review fails:

- Fix critical and important issues.
- Re-run tests and benchmarks affected by the change.
- Re-run quality review.

If performance review finds a regression:

- Confirm the benchmark is valid.
- Profile the regression.
- Either fix it or document why the regression is accepted.
- Do not silently merge performance regressions.

### 6.8 Parallel Subagent Rules

Parallel implementation is allowed only when tasks do not touch the same files or tightly coupled systems.

Safe parallel examples:

- `ChunkCoord` math in `alchimera_core` and README docs.
- Inventory stack tests and Bevy window bootstrap.
- Benchmark harness setup and material definition schema.

Unsafe parallel examples:

- Two agents editing `src/main.rs`.
- Two agents designing the same world generation API.
- One agent refactoring inventory while another adds inventory features.
- One agent changing save data while another writes save/load tests.

If in doubt, serialize the tasks.

### 6.9 Subagent Completion Is Not Verification

A subagent saying "tests pass" is useful but not sufficient for important changes.

The orchestrator should verify by running commands directly, especially before merge:

```bash
git status --short
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

For performance-sensitive changes, verify benchmark reports exist and are plausible.

---

## 7. Implementation Planning Workflow

### 7.1 Plan Before Building

Before implementing a milestone, create a plan under:

```text
docs/plans/YYYY-MM-DD-milestone-name.md
```

Each plan must include:

- Goal.
- Architecture summary.
- Affected files.
- Task list.
- TDD requirements per task.
- Performance measurement requirements.
- Worktree/branch strategy.
- Review gates.
- Manual playtest checklist if visual/gameplay behavior is affected.

### 7.2 Task Format

Use this task format:

```markdown
### Task N: [Small behavior]

**Objective:** [one sentence]

**Worktree:** `/home/openclaw/worktrees/alchimera/[name]`

**Branch:** `feat/[name]`

**Files:**
- Create: `path`
- Modify: `path`
- Test: `path`
- Benchmark: `path` if applicable

**RED:**
- Add test: [test name]
- Run: `cargo test [test name] --workspace -- --nocapture`
- Expected: FAIL because [reason]

**GREEN:**
- Implement minimal code.
- Run: `cargo test [test name] --workspace -- --nocapture`
- Expected: PASS

**Regression:**
- Run: `cargo test -p [crate]`

**Performance:**
- Run: [benchmark command or "not performance-sensitive"]

**Review gates:**
- Spec review.
- Quality/performance review.

**Commit:**
- `git commit -m "feat: [message]"`
```

### 7.3 Milestone Completion Checklist

A milestone is not complete until:

- All tasks are implemented.
- All tests pass.
- Relevant benchmarks are recorded.
- Manual playtest checklist is completed if applicable.
- Docs are updated.
- Worktrees are clean or removed.
- Final integration review passes.

---

## 8. Bevy-Specific Implementation Guidance

### 8.1 Plugin Boundaries

Each feature area should expose a plugin:

```rust
pub struct WorldPlugin;
pub struct PlayerPlugin;
pub struct ObjectPlugin;
pub struct InventoryPlugin;
pub struct CraftingPlugin;
pub struct BuildingPlugin;
pub struct AlchemyPlugin;
pub struct UiPlugin;
```

Plugins should register:

- Components.
- Events.
- Resources.
- Systems.
- System sets.
- Diagnostics.

Keep plugin responsibilities narrow.

### 8.2 Schedule Discipline

Use explicit system sets for ordering.

Example conceptual sets:

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    Input,
    Simulation,
    Interaction,
    Streaming,
    Spawning,
    Despawning,
    Ui,
}
```

Avoid relying on accidental system order.

### 8.3 Events and Commands

Use events for gameplay actions:

- `HarvestEvent`
- `CraftEvent`
- `PlaceObjectEvent`
- `DamageObjectEvent`
- `ChunkRequestedEvent`
- `ChunkGeneratedEvent`

This helps tests and future multiplayer.

### 8.4 Avoid Per-Frame Heavy Work

Do not do these every frame unless measured and justified:

- Regenerate terrain meshes.
- Rebuild all colliders.
- Scan every world object.
- Allocate large vectors in hot systems.
- Query broad sets and filter manually when a narrower query works.
- Deserialize data files.
- Spawn/despawn thousands of entities in one frame.

Prefer:

- Dirty flags.
- Events.
- Spatial partitioning.
- Chunk-local indexes.
- Background tasks.
- Batched spawning/despawning over several frames.

### 8.5 Async and Background Generation

Chunk generation should eventually happen off the main thread.

Design flow:

1. Main thread requests chunk.
2. Worker generates pure data: terrain mesh data, object instance data, collider data descriptors.
3. Worker sends result back.
4. Main thread creates Bevy assets/entities in a bounded budget.

Do not mutate Bevy `World` from background tasks.

### 8.6 Deterministic Generation Boundary

Keep generation code pure where possible.

Good:

```rust
fn generate_chunk(seed: WorldSeed, coord: ChunkCoord, settings: &GenerationSettings) -> GeneratedChunk
```

Bad:

```rust
fn generate_chunk(commands: &mut Commands, assets: &mut Assets<Mesh>)
```

The pure function is testable, benchmarkable, and can run off-thread.

The Bevy system should only adapt generated data into entities/assets.

---

## 9. Code Quality Standards

### 9.1 Rust Standards

Required before merge:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Use:

- Clear newtypes for IDs.
- Explicit seeds.
- Small structs.
- `Result` for recoverable failures.
- `thiserror` or similar when error complexity grows.
- `serde` for persisted data types.

Avoid:

- `unwrap()` in production code unless invariant is documented and unavoidable.
- Hidden global RNG.
- Stringly typed IDs deep in hot paths.
- Large monolithic systems.
- Overly generic abstractions before there are multiple real use cases.

### 9.2 ID Standards

Use stable IDs for generated objects and save/load.

IDs should derive from:

- World seed.
- Chunk coordinate.
- Local generation index or deterministic local key.
- Prototype ID.

Object IDs must not depend on entity IDs, spawn order, or memory addresses.

### 9.3 Error Handling

Errors that can happen due to data or user environment should be represented:

- Invalid object prototype.
- Invalid material definition.
- Recipe references missing item/material.
- Save version unsupported.
- Asset loading failure.

Fail fast in tests and development, but expose meaningful errors.

### 9.4 Data Files

Data-driven files must be validated.

Add tests that load and validate:

- Materials.
- Object prototypes.
- Recipes.
- Biomes.
- Alchemy traits.

Validation should catch:

- Duplicate IDs.
- Missing references.
- Invalid numeric ranges.
- Unused required fields.

---

## 10. Commit Standards

### 10.1 Commit Frequency

Commit after each small task once:

- Tests pass.
- Formatting passes.
- Clippy passes for affected scope or whole workspace.
- Benchmarks are recorded if relevant.

### 10.2 Commit Message Format

Use conventional commits:

```text
feat: add chunk coordinate conversion
fix: preserve harvested object state after reload
perf: reduce object placement allocations
refactor: split generation settings from world plugin
test: add inventory stack merge coverage
docs: add implementation guidelines
```

For TDD feature commits, mention tests if helpful:

```text
feat: add deterministic chunk coordinates

- adds negative-coordinate coverage
- adds world-position conversion tests
```

For performance commits, include numbers:

```text
perf: cache biome noise samples per chunk

Benchmark: chunk_generation/origin
- before: 14.2 ms mean
- after: 8.6 ms mean
```

---

## 11. Review Checklists

### 11.1 Spec Compliance Review

Check:

- Does the implementation satisfy the exact task?
- Does it align with `SPEC.md`?
- Are required files present?
- Are required tests present?
- Are required benchmarks/profiles present?
- Did the task avoid scope creep?
- Is the architecture compatible with future multiplayer where relevant?
- Are deterministic systems actually deterministic?

Verdict format:

```text
Verdict: PASS or REQUEST_CHANGES

Missing requirements:
- ...

Scope creep:
- ...

Required fixes:
- ...
```

### 11.2 Code Quality Review

Check:

- Rust idioms.
- Bevy ECS idioms.
- Test clarity.
- Error handling.
- Determinism.
- Data ownership.
- Performance hazards.
- Naming.
- Module boundaries.
- Serialization compatibility.

Verdict format:

```text
Verdict: APPROVED or REQUEST_CHANGES

Critical issues:
- ...

Important issues:
- ...

Minor issues:
- ...

Commands run:
- ...
```

### 11.3 Performance Review

Check:

- Was a relevant benchmark run?
- Is the benchmark representative?
- Was it run in release mode?
- Are before/after numbers recorded?
- Are allocations reasonable?
- Is work happening on the main thread unnecessarily?
- Does the change increase active entity/collider count?
- Is there a new unbounded cache?

Verdict format:

```text
Verdict: APPROVED or REQUEST_CHANGES

Measured impact:
- ...

Regression risk:
- ...

Required fixes:
- ...
```

---

## 12. Example End-to-End Task

### Task: Add `ChunkCoord` Conversion

#### Worktree

```bash
mkdir -p /home/openclaw/worktrees/alchimera

git worktree add \
  -b feat/chunk-coordinates \
  /home/openclaw/worktrees/alchimera/chunk-coordinates

cd /home/openclaw/worktrees/alchimera/chunk-coordinates
```

#### RED

Create tests for:

- Origin maps to `(0, 0)`.
- Positive coordinates map correctly.
- Negative coordinates floor correctly.
- Boundary values map correctly.

Run:

```bash
cargo test chunk_coord --workspace -- --nocapture
```

Expected:

- FAIL because `ChunkCoord` or conversion function does not exist yet.

#### GREEN

Implement:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn from_world_xz(world_x: f32, world_z: f32, chunk_size: f32) -> Self {
        Self {
            x: (world_x / chunk_size).floor() as i32,
            z: (world_z / chunk_size).floor() as i32,
        }
    }
}
```

Run:

```bash
cargo test chunk_coord --workspace -- --nocapture
cargo test --workspace
```

#### Performance

No benchmark required unless conversion is later found hot in profiling.

#### Review

Spec reviewer checks that this matches invisible chunk partitioning in `SPEC.md`.

Quality reviewer checks negative coordinate behavior, float boundary tests, and naming.

#### Commit

```bash
git add -A
git commit -m "feat: add chunk coordinate conversion"
```

---

## 13. Manual Playtest Protocol

Automated tests are required, but 3D game feel still needs manual playtesting.

For any player-facing feature, record a checklist under:

```text
docs/playtests/YYYY-MM-DD-feature-name.md
```

Template:

```markdown
# Playtest: [Feature]

Date: YYYY-MM-DD
Branch:
Build command:
Run command:
Hardware:

## Checklist

- [ ] Game boots.
- [ ] No console errors.
- [ ] FPS remains acceptable.
- [ ] Feature is discoverable.
- [ ] Controls feel responsive.
- [ ] Visual feedback is clear.
- [ ] Audio feedback is clear if applicable.
- [ ] Save/load still works if applicable.

## Notes

## Bugs Found

## Follow-up Tasks
```

Manual playtests should create automated regression tests for any bug discovered.

---

## 14. Definition of Done

A task is done when:

- It was implemented in an isolated branch/worktree.
- Tests were written before production code.
- The failing test was observed.
- The passing test was observed.
- Relevant broader tests pass.
- Formatting passes.
- Clippy passes.
- Performance was measured if relevant.
- Documentation was updated if behavior or architecture changed.
- Spec review passed.
- Quality/performance review approved.
- Changes are committed.
- The worktree is clean.

A milestone is done when:

- Every task meets the task definition of done.
- Full workspace tests pass.
- Relevant benchmarks are recorded.
- Manual playtest checklist passes where applicable.
- Integration review passes.
- Branches are merged or explicitly abandoned.
- Obsolete worktrees are removed.

---

## 15. Non-Negotiable Rules

1. No production behavior without a failing test first.
2. No procedural generation without determinism tests.
3. No performance-sensitive system without measurement.
4. No subagent implementation without review.
5. No parallel subagents editing the same files.
6. No direct large changes on `main`.
7. No hidden global randomness in generation.
8. No unbounded caches without eviction strategy.
9. No save format changes without round-trip tests.
10. No merge while `cargo fmt`, `cargo clippy`, or `cargo test` are failing.

---

## 16. Initial Recommended First Tasks

Start with foundational tasks in this order:

1. Bootstrap Bevy app.
2. Add workspace/crate structure.
3. Add CI-equivalent local commands in README.
4. Add `WorldSeed` and deterministic seed derivation tests.
5. Add `ChunkCoord` conversion tests and implementation.
6. Add generation settings data type.
7. Add basic height sampler with determinism tests.
8. Add benchmark harness for height sampling.
9. Add terrain mesh data generation tests.
10. Add minimal Bevy terrain rendering adapter.

Do not begin complex player interaction, crafting, alchemy, or building until seed/chunk/generation foundations are tested and benchmarked.
