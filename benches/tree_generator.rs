use alchimera_core::seed::WorldSeed;
use alchimera_generation::tree::{generate_tree, TreeConfig};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_tree_generator(c: &mut Criterion) {
    c.bench_function("generate_tree_default", |b| {
        b.iter(|| {
            let tree = generate_tree(
                black_box(WorldSeed::new(42)),
                black_box(TreeConfig::default()),
            );
            black_box(tree.summary());
        });
    });
}

criterion_group!(benches, bench_tree_generator);
criterion_main!(benches);
