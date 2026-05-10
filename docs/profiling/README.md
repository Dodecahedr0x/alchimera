# Profiling and Benchmarks

Alchimera uses [Criterion](https://bheisler.github.io/criterion.rs/book/) for Rust microbenchmarks.

Run the smoke benchmark with:

```sh
cargo bench --bench smoke
```

Criterion writes benchmark output under `target/criterion/`. The smoke benchmark report is generated at:

```text
target/criterion/build_minimal_bevy_app/report/index.html
```

Open that HTML report in a browser to inspect timing distributions and historical comparisons.