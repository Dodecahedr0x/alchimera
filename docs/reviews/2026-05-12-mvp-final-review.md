# MVP Vertical Slice Final Integration Review — 2026-05-12

## Verdict

**PASS with follow-up manual validation required.**

The repository now contains the planned MVP vertical-slice feature set, automated tests cover the core deterministic/data/ECS contracts, and the standard quality gates pass in a clean worktree. The remaining gap is not a code blocker: true manual playtest observations, live FPS/frame-time, and live entity-count measurements still need to be collected from an interactive display session rather than headless cron.

## Scope reviewed

Reviewed against:

- `SPEC.md`
- `IMPLEMENTATION_GUIDELINES.md`
- `docs/plans/parallel-subagent-task-breakdown.md`
- Current `main` as of the W5-T2/W5-T3 merges

Relevant final-wave artifacts:

- `docs/playtests/2026-05-12-mvp-vertical-slice.md`
- `docs/profiling/2026-05-12-mvp-baseline.md`

## Commands run

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Result: all passed.

Additional integration evidence from the previous W5-T1/W5-T3 pass:

```bash
cargo test --workspace --release
cargo bench --workspace
cargo run            # boot smoke, timed out after entering Bevy run loop
cargo run --release  # attempted from W5-T3 worktree; timed out during compile/headless run setup
```

## Review checks

### MVP vertical slice coverage

**Status: PASS for automated slice contracts; manual playtest still pending.**

Implemented/tested areas present in the workspace include:

- Deterministic seed, IDs, chunk coordinates, terrain height, biome selection, terrain mesh generation.
- Procedural object prototypes, object placement, tree and rock generator data.
- Bevy app shell, app states, player/camera input scaffolding, diagnostics, terrain rendering, object spawning, chunk streaming.
- Interaction targeting, harvesting, inventory/hotbar shell, crafting, basic building placement, save/load persistence, and alchemy core logic.

The playtest checklist exists and explicitly marks headless-cron limitations instead of falsely claiming visual/input completion.

### Tests and TDD alignment

**Status: PASS.**

The workspace has focused integration tests by subsystem, including:

- Core: seed, IDs, material/item definitions, inventory, crafting, save data, validation, alchemy.
- Generation: chunks, biomes, height sampling, mesh data, object generation/placement/prototypes/rendering, tree/rock generators, modification log.
- Game/ECS: app state, terrain rendering, player, input/camera, diagnostics, object spawning, chunk streaming, interaction, harvesting, inventory UI, crafting systems, building, persistence.
- Viewer: procedural object visualizer output tests.

The tests assert behavior and structural contracts rather than screenshots or implementation-only details, which matches the plan and guidelines.

### Deterministic generation guarantees

**Status: PASS.**

Determinism is covered for seed derivation, biome selection, height sampling, terrain mesh generation, object generation/placement, tree generation, rock generation, stable object IDs, and chunk modification-log application.

### Performance measurements

**Status: PASS for generation microbenchmarks; NEEDS FOLLOW-UP for live runtime metrics.**

The performance baseline records benchmark means for:

- Biome samples
- Height samples
- Terrain mesh generation
- Chunk object placement
- Object placement
- Tree generation
- Rock generation
- Minimal Bevy app construction

Known gap: FPS/frame-time and live entity count were not observed because the cron environment is headless. This is documented in both the playtest checklist and baseline report.

### ECS modularity

**Status: PASS.**

The Bevy-facing systems are split into focused `alchimera_game` modules and tested via observable resources/components/events. Pure domain logic remains in `alchimera_core` and `alchimera_generation`, keeping most deterministic rules engine-agnostic.

### Save/load tests

**Status: PASS.**

Persistence tests cover inventory, harvested-object overrides, and placed objects. Core save-data tests cover versioning and JSON roundtrip behavior.

### Worktrees and branches

**Status: PASS for active final-review worktree.**

This review worktree is based on current `origin/main`. Historical task worktrees remain present for traceability, but the main checkout was clean and synced after W5-T2/W5-T3 merges were pushed.

## Important follow-ups

1. Run `docs/playtests/2026-05-12-mvp-vertical-slice.md` from an interactive machine with a display and input.
2. Record actual runtime FPS/frame-time and entity count in a follow-up profiling note.
3. Investigate the Criterion-reported `build_minimal_bevy_app` regression if it reproduces in the next local benchmark run.
4. Extend the new scripted non-windowed runtime metrics mode to simulate seeded player movement and streamed chunk traversal, so future cron jobs can collect richer runtime entity/frame data without manual play.

## Conclusion

The automated MVP vertical-slice integration is in good shape and `main` is shippable for the next manual playtest/profiling pass. No code changes are required from this review before continuing to manual runtime validation.
