//! Adapters from pure generation mesh data into Bevy world entities.

use alchimera_core::seed::WorldSeed;
use alchimera_generation::{
    chunk::ChunkCoord,
    mesh::{generate_terrain_mesh, TerrainMeshConfig, TerrainMeshData},
    terrain::TerrainConfig,
};
use bevy::{
    asset::{Assets, Handle},
    prelude::{Component, Entity, Transform, World},
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
    let data = generate_terrain_mesh(seed, chunk, terrain_config, mesh_config);
    let mesh = terrain_mesh_data_to_bevy_mesh(&data);
    let handle = world.resource_mut::<Assets<Mesh>>().add(mesh);
    world
        .spawn((
            TerrainChunk { coord: chunk },
            TerrainMeshHandle { mesh: handle },
            Transform::default(),
        ))
        .id()
}
