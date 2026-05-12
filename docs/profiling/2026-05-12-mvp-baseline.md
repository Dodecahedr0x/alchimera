# MVP Vertical Slice Performance Baseline — 2026-05-12

## Scope

This is the initial baseline for the MVP vertical slice after Wave 4 features were merged through `main` and the `integration/vertical-slice` branch was created.

Environment: scheduled headless cron run on Linux. Automated benchmark commands were run from the repository; interactive GPU/windowed runtime measurements could not be visually observed in this environment.

## Commands run

```bash
cargo test --workspace --release
cargo bench --workspace
cargo run --release   # attempted under timeout in headless cron/worktree
```

Results:

- `cargo test --workspace --release`: passed.
- `cargo bench --workspace`: passed.
- `cargo run --release`: attempted in the W5-T3 worktree with a timeout. The command was still compiling Bevy/runtime crates when the timeout expired, so no release runtime window/FPS observation was collected in this cron pass.

## Generation benchmark means

From `cargo bench --workspace`:

| Benchmark | Mean / range reported |
| --- | ---: |
| `chunk_biome_samples` | 279.19 ns – 285.31 ns |
| `chunk_height_samples` | 687.24 ns – 708.72 ns |
| `chunk_terrain_mesh` | 38.495 µs – 39.493 µs |
| `chunk_object_placement` | 8.5008 µs – 8.7059 µs |
| `object_placement_one_chunk` | 8.3740 µs – 8.8040 µs |
| `generate_rock_default` | 3.1565 µs – 3.3265 µs |
| `build_minimal_bevy_app` | 222.81 µs – 238.64 µs |
| `generate_tree_default` | 1.4541 µs – 1.5143 µs |

Criterion notes:

- `Gnuplot not found`; Criterion used the plotters backend.
- `build_minimal_bevy_app` reported a regression versus prior Criterion history: `+80.818% +86.448% +92.440%`.

## Active chunk radius tested

Automated benchmarks exercise one generated chunk or a minimal app setup, depending on the benchmark. The runtime chunk-streaming tests cover the configured active radius behavior at the ECS/system level, but this baseline did not perform an interactive moving-player runtime session.

For this baseline, record the practical automated scope as:

- Generation microbenchmarks: one chunk / one generator invocation.
- Runtime visual play session: not observed in headless cron.

## FPS / frame-time observed

Not observed.

Reason: the scheduled cron environment has no interactive display/input session. The release runtime command was attempted with a timeout but did not reach a usable observed window before timing out while compiling in the separate worktree.

Next measurement should be done from an interactive machine/session with a display and the diagnostics overlay enabled.

## Entity count observed

Not observed in a live runtime session for the same headless cron reason.

Automated ECS tests verify terrain, camera/light, generated object, inventory, crafting, harvesting, building, persistence, and chunk-streaming state in isolation, but this report should not claim a live world entity count until a manual/interactive run records one.

## Known bottlenecks / risks

- Runtime FPS and entity-count data are still missing; the automated suite validates behavior, not visual/runtime throughput.
- `build_minimal_bevy_app` benchmark regressed relative to local Criterion history and should be investigated if startup cost matters for the MVP.
- Generation microbenchmarks are fast at one-chunk scope, but they do not yet measure sustained streaming over an active chunk radius while objects and gameplay systems are active.
- Benchmark runs reported several outliers, especially object placement and app construction; future baselines should keep the same hardware/session and compare Criterion reports over time.

## Next performance tasks

1. Run the MVP manually with a display and diagnostics overlay, then record:
   - FPS / frame time.
   - Entity count.
   - Active chunk radius.
   - Seed and player path used.
2. Add or extend runtime benchmarks for chunk streaming over multiple chunks rather than only one-chunk generator microbenchmarks.
3. Investigate the `build_minimal_bevy_app` Criterion regression if it reproduces on the next local benchmark run.
4. Add a repeatable scripted runtime smoke mode if possible so cron can gather non-visual runtime metrics without a windowed playtest.
