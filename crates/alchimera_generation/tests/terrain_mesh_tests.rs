use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    mesh::{generate_terrain_mesh, TerrainMeshConfig, TerrainMeshGenerator},
    terrain::TerrainConfig,
};

#[test]
fn grid_resolution_controls_vertex_and_index_counts() {
    let mesh = generate_terrain_mesh(
        WorldSeed::new(42),
        ChunkCoord::new(0, 0),
        TerrainConfig::new(0.0, 10.0),
        TerrainMeshConfig::new(4),
    );

    assert_eq!(mesh.positions().len(), 25);
    assert_eq!(mesh.normals().len(), 25);
    assert_eq!(mesh.uvs().len(), 25);
    assert_eq!(mesh.indices().len(), 96);
}

#[test]
fn normals_reflect_sampled_height_slopes() {
    let mesh = generate_terrain_mesh(
        WorldSeed::new(7),
        ChunkCoord::new(2, -1),
        TerrainConfig::new(-20.0, 40.0),
        TerrainMeshConfig::new(8),
    );

    assert!(mesh.normals().iter().all(|normal| {
        let length = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
        (length - 1.0).abs() <= 0.001
    }));
    assert!(
        mesh.normals()
            .iter()
            .any(|normal| normal[0].abs() > 0.001 || normal[2].abs() > 0.001),
        "expected at least one normal to tilt with terrain slope"
    );
}

#[test]
fn terrain_mesh_generator_is_deterministic_for_same_seed_and_chunk() {
    let generator = TerrainMeshGenerator::new(TerrainConfig::default(), TerrainMeshConfig::new(8));
    let seed = WorldSeed::new(0xA1C0_E011);
    let chunk = ChunkCoord::new(-4, 3);

    let first = generator.generate_chunk(seed, chunk);
    let second = generator.generate_chunk(seed, chunk);

    assert_eq!(first, second);
}
