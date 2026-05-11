# Generation Benchmark Baseline — 2026-05-11

## Scope

Initial consolidated generation benchmark suite for one deterministic chunk.

Covered paths:

- Biome sampling at representative local chunk points.
- Height sampling at representative local chunk points.
- Terrain mesh data generation for one chunk with 16 subdivisions.
- Deterministic object placement for one chunk.

## Commands

```bash
cargo bench --bench chunk_generation -- --test
cargo test --workspace
```

## Baseline notes

The `chunk_generation` Criterion target is intended to be the stable entry point for generation performance tracking. The first verification run used Criterion's `--test` mode to compile and execute the benchmark functions quickly in CI/agent workflows without recording statistical timing samples.

Future timing runs should use:

```bash
cargo bench --bench chunk_generation
```

and append Criterion estimates for each benchmark function:

- `chunk_biome_samples`
- `chunk_height_samples`
- `chunk_terrain_mesh`
- `chunk_object_placement`

## Acceptance

- Benchmarks cover biome, height, terrain mesh, and object placement for one chunk.
- Workspace tests remain green after adding the benchmark target.
