use alchimera_core::seed::WorldSeed;
use alchimera_generation::rock::{generate_rock, RockConfig};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_rock_generator(c: &mut Criterion) {
    c.bench_function("generate_rock_default", |b| {
        b.iter(|| {
            let rock = generate_rock(
                black_box(WorldSeed::new(42)),
                black_box(RockConfig::default()),
            );
            black_box(rock.summary());
        });
    });
}

criterion_group!(benches, bench_rock_generator);
criterion_main!(benches);
