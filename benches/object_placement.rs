use alchimera_core::seed::WorldSeed;
use alchimera_generation::{chunk::ChunkCoord, objects::ObjectGenerator};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_object_placement(c: &mut Criterion) {
    let generator = ObjectGenerator::default();
    let seed = WorldSeed::new(0x000B_1EC7_0001);
    let chunk = ChunkCoord::new(0, 0);

    c.bench_function("object_placement_one_chunk", |b| {
        b.iter(|| {
            let objects = generator.generate_chunk(black_box(seed), black_box(chunk));
            black_box(objects);
        });
    });
}

criterion_group!(benches, bench_object_placement);
criterion_main!(benches);
