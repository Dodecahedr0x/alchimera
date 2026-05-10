# Alchimera Data Assets

This directory contains data-driven definitions used by Alchimera's procedural world, crafting, and alchemy systems.

## Directories

- `materials/`: material definitions such as wood, stone, ore, leaves, soil, and alchemical traits.
- `objects/`: object prototype definitions such as trees, rocks, plants, ruins, stations, and building pieces.
- `recipes/`: crafting and station recipe definitions.
- `biomes/`: biome configuration and spawn/distribution rules.
- `alchemy/`: alchemical trait, process, reagent, and discovery definitions.

## Ownership Rules

- Data files should be deterministic inputs, not generated runtime output.
- Runtime-generated files belong under `target/` or a future save/cache directory, not here.
- Every data format added here must have validation tests before production systems depend on it.
- IDs must be namespaced and stable, for example `wood.oak`, `tree.oak.mature`, or `recipe.tool.stone_axe`.
- Cross-references must be validated: object prototypes should reference existing materials, recipes should reference existing items/material classes, and alchemy definitions should reference existing traits.

## Expected Future Format

The current placeholder layout is intentionally minimal. Future tasks may add RON, TOML, or another serde-supported format once the validation layer exists.
