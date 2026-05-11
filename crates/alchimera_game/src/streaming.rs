//! Chunk streaming resources and systems.

use std::collections::HashSet;

use alchimera_generation::chunk::ChunkCoord;
use bevy::prelude::{
    App, Component, Plugin, Query, Res, ResMut, Resource, Transform, Update, With,
};

use crate::{diagnostics::RuntimeDiagnostics, player::Player};

/// Runtime marker for an active streamed chunk entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct ActiveChunk {
    pub coord: ChunkCoord,
}

/// Configuration for player-centered chunk streaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub struct ChunkStreamingConfig {
    pub active_radius_chunks: i32,
}

impl Default for ChunkStreamingConfig {
    fn default() -> Self {
        Self {
            active_radius_chunks: 1,
        }
    }
}

/// Observable chunk streaming state: queued requests and despawn marks.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct ChunkStreaming {
    requested_chunks: HashSet<ChunkCoord>,
    despawn_chunks: HashSet<ChunkCoord>,
}

impl ChunkStreaming {
    #[must_use]
    pub fn requested_chunks(&self) -> &HashSet<ChunkCoord> {
        &self.requested_chunks
    }

    #[must_use]
    pub fn despawn_chunks(&self) -> &HashSet<ChunkCoord> {
        &self.despawn_chunks
    }

    #[must_use]
    pub fn is_requested(&self, coord: ChunkCoord) -> bool {
        self.requested_chunks.contains(&coord)
    }

    #[must_use]
    pub fn is_marked_for_despawn(&self, coord: ChunkCoord) -> bool {
        self.despawn_chunks.contains(&coord)
    }
}

/// Registers player-centered chunk streaming systems.
#[derive(Debug, Default)]
pub struct ChunkStreamingPlugin;

impl Plugin for ChunkStreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkStreamingConfig>()
            .init_resource::<ChunkStreaming>()
            .add_systems(Update, update_chunk_streaming);
    }
}

fn update_chunk_streaming(
    config: Res<ChunkStreamingConfig>,
    mut streaming: ResMut<ChunkStreaming>,
    mut diagnostics: ResMut<RuntimeDiagnostics>,
    players: Query<&Transform, With<Player>>,
    active_chunks: Query<&ActiveChunk>,
) {
    let Some(player_transform) = players.iter().next() else {
        return;
    };

    let center = ChunkCoord::from_world_position(
        player_transform.translation.x,
        player_transform.translation.z,
    );
    streaming.requested_chunks.clear();
    streaming.despawn_chunks.clear();

    let radius = config.active_radius_chunks;
    for z in (center.z - radius)..=(center.z + radius) {
        for x in (center.x - radius)..=(center.x + radius) {
            streaming.requested_chunks.insert(ChunkCoord::new(x, z));
        }
    }

    for active in &active_chunks {
        if (active.coord.x - center.x).abs() > radius || (active.coord.z - center.z).abs() > radius
        {
            streaming.despawn_chunks.insert(active.coord);
        }
    }

    diagnostics.active_chunks = active_chunks.iter().count();
    diagnostics.queued_chunks = streaming.requested_chunks.len();
}
