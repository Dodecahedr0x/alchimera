use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    biome::sample_biome,
    chunk::{ChunkCoord, CHUNK_SIZE_METERS},
    mesh::{TerrainMeshConfig, TerrainMeshGenerator},
    objects::ObjectGenerator,
    terrain::{sample_height, TerrainConfig},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SAMPLE_POINTS: [(f32, f32); 5] = [
    (0.0, 0.0),
    (CHUNK_SIZE_METERS * 0.25, CHUNK_SIZE_METERS * 0.25),
    (CHUNK_SIZE_METERS * 0.5, CHUNK_SIZE_METERS * 0.5),
    (CHUNK_SIZE_METERS * 0.75, CHUNK_SIZE_METERS * 0.75),
    (CHUNK_SIZE_METERS, CHUNK_SIZE_METERS),
];

fn bench_generation_pipeline(c: &mut Criterion) {
    let seed = WorldSeed::new(0xC0FF_EE00_0001);
    let chunk = ChunkCoord::new(0, 0);
    let terrain_config = TerrainConfig::default();
    let mesh_generator = TerrainMeshGenerator::new(terrain_config, TerrainMeshConfig::new(16));
    let object_generator = ObjectGenerator::default();

    c.bench_function("chunk_biome_samples", |b| {
        b.iter(|| {
            let biomes = SAMPLE_POINTS.map(|(local_x, local_z)| {
                sample_biome(black_box(seed), black_box(chunk), local_x, local_z)
            });
            black_box(biomes);
        });
    });

    c.bench_function("chunk_height_samples", |b| {
        b.iter(|| {
            let heights = SAMPLE_POINTS.map(|(local_x, local_z)| {
                sample_height(
                    black_box(seed),
                    black_box(chunk),
                    local_x,
                    local_z,
                    black_box(terrain_config),
                )
            });
            black_box(heights);
        });
    });

    c.bench_function("chunk_terrain_mesh", |b| {
        b.iter(|| {
            let mesh = mesh_generator.generate_chunk(black_box(seed), black_box(chunk));
            black_box((mesh.positions().len(), mesh.indices().len()));
        });
    });

    c.bench_function("chunk_object_placement", |b| {
        b.iter(|| {
            let objects = object_generator.generate_chunk(black_box(seed), black_box(chunk));
            black_box(objects.len());
        });
    });
}

criterion_group!(benches, bench_generation_pipeline);
criterion_main!(benches);
