use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_build_app(c: &mut Criterion) {
    c.bench_function("build_minimal_bevy_app", |b| {
        b.iter(|| {
            let app = alchimera_game::build_app();
            black_box(app);
        });
    });
}

criterion_group!(benches, bench_build_app);
criterion_main!(benches);
