# Alchimera

Alchimera is a Rust/Bevy 3D open-world survival sandbox where procedural, editable objects replace visible cube blocks.

## Project Documents

- [`SPEC.md`](SPEC.md): game vision, design pillars, technical requirements, and milestone scope.
- [`IMPLEMENTATION_GUIDELINES.md`](IMPLEMENTATION_GUIDELINES.md): TDD workflow, performance expectations, repository practices, and review rules.
- [`docs/plans/parallel-subagent-task-breakdown.md`](docs/plans/parallel-subagent-task-breakdown.md): wave plan, task ownership, worktree convention, and review gates for parallel development.

## Development Commands

Run commands from the repository root unless noted otherwise.

### Format

```bash
cargo fmt --all -- --check
```

### Test

```bash
cargo test --workspace
```

### Lint

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Build

```bash
cargo build --workspace
```

### Run

```bash
cargo run
```

### Benchmarks

Run all available benchmarks:

```bash
cargo bench
```

Run a targeted benchmark when one is added:

```bash
cargo bench --bench <bench_name>
```

## Worktree Workflow

Use isolated branches and worktrees for each feature, bug fix, experiment, or subagent task.

Example:

```bash
mkdir -p /home/openclaw/worktrees/alchimera

git worktree add \
  -b docs/developer-commands \
  /home/openclaw/worktrees/alchimera/w0-t2-readme

cd /home/openclaw/worktrees/alchimera/w0-t2-readme
```

Subagents should work only inside their assigned worktree and report files changed, commands run, and the final commit hash.
