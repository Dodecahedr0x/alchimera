//! Headless runtime metrics collection for cron/CI environments.

use std::{collections::HashSet, time::Instant};

use alchimera_generation::chunk::{ChunkCoord, CHUNK_SIZE_METERS};
use bevy::prelude::{Transform, Vec3, With};

use crate::{build_app, diagnostics::RuntimeDiagnostics, player::Player};

/// Summary produced by a bounded headless app-update run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadlessMetricsReport {
    /// Number of Bevy update frames executed.
    pub frames: u32,
    /// Wall-clock time spent running those frames, in microseconds.
    pub elapsed_micros: u128,
    /// Latest diagnostics resource sampled from the app.
    pub metrics: RuntimeDiagnostics,
    /// Live entity count observed directly from the Bevy world after the run.
    pub entity_count: usize,
    /// Player chunk observed before scripted traversal begins.
    pub initial_player_chunk: ChunkCoord,
    /// Player chunk observed after the final sampled frame.
    pub final_player_chunk: ChunkCoord,
    /// Number of distinct player chunks sampled during the run.
    pub unique_player_chunks: usize,
}

/// Runs the minimal game app for a bounded number of update frames without opening a window.
#[must_use]
pub fn run_headless_metrics(requested_frames: u32) -> HeadlessMetricsReport {
    run_headless_metrics_with_traversal(requested_frames, None)
}

/// Runs a deterministic, seeded traversal sample through the headless runtime.
#[must_use]
pub fn run_headless_traversal_metrics(
    requested_frames: u32,
    movement_seed: u64,
) -> HeadlessMetricsReport {
    run_headless_metrics_with_traversal(requested_frames, Some(movement_seed))
}

fn run_headless_metrics_with_traversal(
    requested_frames: u32,
    movement_seed: Option<u64>,
) -> HeadlessMetricsReport {
    let frames = requested_frames.max(1);
    let mut app = build_app();

    // Run startup once so the player entity exists before scripted movement samples begin.
    app.update();
    let initial_player_chunk = player_chunk(&mut app).unwrap_or(ChunkCoord::new(0, 0));
    let mut sampled_chunks = HashSet::from([initial_player_chunk]);

    let start = Instant::now();
    for frame in 0..frames {
        if let Some(seed) = movement_seed {
            let translation = seeded_player_translation(seed, frame);
            set_player_translation(&mut app, translation);
        }
        app.update();
        if let Some(chunk) = player_chunk(&mut app) {
            sampled_chunks.insert(chunk);
        }
    }
    let elapsed_micros = start.elapsed().as_micros().max(1);

    let metrics = *app.world().resource::<RuntimeDiagnostics>();
    let entity_count = app.world().entities().len() as usize;
    let final_player_chunk = player_chunk(&mut app).unwrap_or(initial_player_chunk);

    HeadlessMetricsReport {
        frames,
        elapsed_micros,
        metrics,
        entity_count,
        initial_player_chunk,
        final_player_chunk,
        unique_player_chunks: sampled_chunks.len(),
    }
}

fn seeded_player_translation(seed: u64, frame: u32) -> Vec3 {
    let direction = if seed.is_multiple_of(2) { 1.0 } else { -1.0 };
    let stride = CHUNK_SIZE_METERS + 8.0;
    let x = direction * (frame as f32 + 1.0) * stride;
    let z_bias = ((seed % 5) as f32 - 2.0) * 4.0;
    Vec3::new(x, 0.0, z_bias)
}

fn set_player_translation(app: &mut bevy::prelude::App, translation: Vec3) {
    let mut query = app
        .world_mut()
        .query_filtered::<&mut Transform, With<Player>>();
    for mut transform in query.iter_mut(app.world_mut()) {
        transform.translation = translation;
    }
}

fn player_chunk(app: &mut bevy::prelude::App) -> Option<ChunkCoord> {
    let mut query = app.world_mut().query_filtered::<&Transform, With<Player>>();
    query.iter(app.world()).next().map(|transform| {
        ChunkCoord::from_world_position(transform.translation.x, transform.translation.z)
    })
}
