use alchimera_core::seed::WorldSeed;
use alchimera_game::terrain_rendering::{
    spawn_terrain_chunk, terrain_mesh_data_to_bevy_mesh, TerrainChunk, TerrainMeshHandle,
};
use alchimera_generation::{
    chunk::ChunkCoord,
    mesh::{generate_terrain_mesh, TerrainMeshConfig},
    terrain::TerrainConfig,
};
use bevy::prelude::{Assets, Mesh};

#[test]
fn terrain_mesh_data_converts_to_bevy_mesh_with_same_vertex_count() {
    let data = generate_terrain_mesh(
        WorldSeed::new(13),
        ChunkCoord::new(0, 0),
        TerrainConfig::default(),
        TerrainMeshConfig::new(4),
    );

    let mesh = terrain_mesh_data_to_bevy_mesh(&data);

    assert_eq!(mesh.count_vertices(), data.positions().len());
    assert_eq!(
        mesh.indices().expect("terrain mesh indices").len(),
        data.indices().len()
    );
}

#[test]
fn spawn_terrain_chunk_adds_chunk_marker_component() {
    let mut world = bevy::prelude::World::new();
    world.init_resource::<Assets<Mesh>>();
    let chunk = ChunkCoord::new(-2, 5);

    let entity = spawn_terrain_chunk(
        &mut world,
        WorldSeed::new(99),
        chunk,
        TerrainConfig::default(),
        TerrainMeshConfig::new(2),
    );

    let entity_ref = world.entity(entity);
    assert_eq!(entity_ref.get::<TerrainChunk>().unwrap().coord, chunk);
    let handle = entity_ref.get::<TerrainMeshHandle>().unwrap();
    assert!(world.resource::<Assets<Mesh>>().get(&handle.mesh).is_some());
}
