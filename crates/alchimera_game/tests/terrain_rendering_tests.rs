use alchimera_core::seed::WorldSeed;
use alchimera_game::terrain_rendering::{
    spawn_terrain_chunk, terrain_mesh_data_to_bevy_mesh, TerrainChunk, TerrainMeshHandle,
};
use alchimera_generation::{
    chunk::ChunkCoord,
    mesh::{generate_terrain_mesh, TerrainMeshConfig},
    terrain::TerrainConfig,
};
use bevy::prelude::{
    Assets, Camera3d, DirectionalLight, Mesh, Mesh3d, MeshMaterial3d, QueryState, StandardMaterial,
};

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

#[test]
fn spawn_terrain_chunk_adds_renderable_3d_mesh_components() {
    let mut world = bevy::prelude::World::new();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();

    let entity = spawn_terrain_chunk(
        &mut world,
        WorldSeed::new(99),
        ChunkCoord::new(0, 0),
        TerrainConfig::default(),
        TerrainMeshConfig::new(2),
    );

    let entity_ref = world.entity(entity);
    let mesh = entity_ref
        .get::<Mesh3d>()
        .expect("renderable mesh component");
    let material = entity_ref
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("renderable material component");

    assert!(world.resource::<Assets<Mesh>>().get(&mesh.0).is_some());
    assert!(world
        .resource::<Assets<StandardMaterial>>()
        .get(&material.0)
        .is_some());
}

#[test]
fn build_app_spawns_3d_camera_and_light() {
    let mut app = alchimera_game::build_app();
    app.update();

    let mut camera_query: QueryState<&Camera3d> = app.world_mut().query();
    let mut light_query: QueryState<&DirectionalLight> = app.world_mut().query();

    assert_eq!(camera_query.iter(app.world()).count(), 1);
    assert_eq!(light_query.iter(app.world()).count(), 1);
}

#[test]
fn build_app_spawns_renderable_terrain_chunk() {
    let mut app = alchimera_game::build_app();
    app.update();

    let mut terrain_query: QueryState<(&TerrainChunk, &Mesh3d, &MeshMaterial3d<StandardMaterial>)> =
        app.world_mut().query();
    let terrain_chunks: Vec<_> = terrain_query.iter(app.world()).collect();

    assert_eq!(terrain_chunks.len(), 1);
}
