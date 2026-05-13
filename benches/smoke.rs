use alchimera_game::runtime_metrics::run_headless_traversal_metrics;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_build_app(c: &mut Criterion) {
    c.bench_function("build_minimal_bevy_app", |b| {
        b.iter(|| {
            let app = alchimera_game::build_app();
            black_box(app);
        });
    });
}

fn bench_chunk_streaming_runtime(c: &mut Criterion) {
    c.bench_function("headless_chunk_streaming_traversal_16_frames", |b| {
        b.iter(|| {
            let report = run_headless_traversal_metrics(black_box(16), black_box(0xA1C0_1E4A));
            black_box((
                report.final_player_chunk,
                report.unique_player_chunks,
                report.metrics.queued_chunks,
            ));
        });
    });
}

criterion_group!(benches, bench_build_app, bench_chunk_streaming_runtime);
criterion_main!(benches);
