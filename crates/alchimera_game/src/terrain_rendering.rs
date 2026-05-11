//! Adapters from pure generation mesh data into Bevy world entities.

use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    mesh::{generate_terrain_mesh, TerrainMeshConfig, TerrainMeshData},
    terrain::TerrainConfig,
};
use bevy::{
    asset::{Assets, Handle},
    prelude::{
        App, Color, Commands, Component, DirectionalLight, Entity, Mesh3d, MeshMaterial3d, Plugin,
        ResMut, StandardMaterial, Startup, Transform, Vec3, World,
    },
    render::{
        mesh::{Indices, Mesh},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

/// Marker component for terrain spawned for a generation chunk.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct TerrainChunk {
    pub coord: ChunkCoord,
}

/// Component storing the Bevy mesh asset handle backing a terrain chunk.
#[derive(Debug, Clone, PartialEq, Component)]
pub struct TerrainMeshHandle {
    pub mesh: Handle<Mesh>,
}

/// Registers the minimal 3D scene required to render generated terrain.
#[derive(Debug, Default)]
pub struct TerrainRenderingPlugin;

impl Plugin for TerrainRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, spawn_initial_render_scene);
    }
}

fn spawn_initial_render_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let chunk = ChunkCoord::new(0, 0);
    let data = generate_terrain_mesh(
        WorldSeed::new(0),
        chunk,
        TerrainConfig::default(),
        TerrainMeshConfig::new(8),
    );
    let mesh = meshes.add(terrain_mesh_data_to_bevy_mesh(&data));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.55, 0.25),
        ..Default::default()
    });

    commands.spawn((
        TerrainChunk { coord: chunk },
        TerrainMeshHandle { mesh: mesh.clone() },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::default(),
    ));
}

#[must_use]
pub fn terrain_mesh_data_to_bevy_mesh(data: &TerrainMeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions().to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals().to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs().to_vec());
    mesh.insert_indices(Indices::U32(data.indices().to_vec()));
    mesh
}

pub fn spawn_terrain_chunk(
    world: &mut World,
    seed: WorldSeed,
    chunk: ChunkCoord,
    terrain_config: TerrainConfig,
    mesh_config: TerrainMeshConfig,
) -> Entity {
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();

    let data = generate_terrain_mesh(seed, chunk, terrain_config, mesh_config);
    let mesh = terrain_mesh_data_to_bevy_mesh(&data);
    let handle = world.resource_mut::<Assets<Mesh>>().add(mesh);
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.55, 0.25),
            ..Default::default()
        });
    world
        .spawn((
            TerrainChunk { coord: chunk },
            TerrainMeshHandle {
                mesh: handle.clone(),
            },
            Mesh3d(handle),
            MeshMaterial3d(material),
            Transform::default(),
        ))
        .id()
}
