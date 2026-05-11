//! Runtime diagnostics resources and plugin wiring.

use bevy::{
    diagnostic::{
        DiagnosticsPlugin as BevyDiagnosticsPlugin, DiagnosticsStore, EntityCountDiagnosticsPlugin,
        FrameTimeDiagnosticsPlugin,
    },
    prelude::{App, Event, EventReader, Plugin, Res, ResMut, Resource, Update},
};

/// Lightweight runtime metrics surfaced by the diagnostics overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub struct RuntimeDiagnostics {
    /// Last sampled frames-per-second value, rounded for display.
    pub fps: Option<u32>,
    /// Last sampled frame time in milliseconds, rounded for display.
    pub frame_time_ms: Option<u32>,
    /// Last sampled Bevy entity count.
    pub entity_count: usize,
    /// Number of terrain chunks currently active in the runtime world.
    pub active_chunks: usize,
    /// Number of chunk work items waiting to be processed.
    pub queued_chunks: usize,
}

/// User-facing diagnostics overlay state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub struct DiagnosticsOverlay {
    /// Whether the diagnostics overlay should be visible.
    pub visible: bool,
    /// Latest sampled diagnostic metrics.
    pub metrics: RuntimeDiagnostics,
}

/// Event that toggles diagnostics overlay visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct ToggleDiagnosticsOverlay;

/// Registers diagnostic resources and low-cost overlay control systems.
#[derive(Debug, Default)]
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BevyDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
        ))
        .init_resource::<RuntimeDiagnostics>()
        .init_resource::<DiagnosticsOverlay>()
        .add_event::<ToggleDiagnosticsOverlay>()
        .add_systems(
            Update,
            (toggle_diagnostics_overlay, sync_runtime_diagnostics),
        );
    }
}

fn toggle_diagnostics_overlay(
    mut events: EventReader<ToggleDiagnosticsOverlay>,
    mut overlay: ResMut<DiagnosticsOverlay>,
) {
    for _ in events.read() {
        overlay.visible = !overlay.visible;
    }
}

fn sync_runtime_diagnostics(
    diagnostics: Res<DiagnosticsStore>,
    mut metrics: ResMut<RuntimeDiagnostics>,
    mut overlay: ResMut<DiagnosticsOverlay>,
) {
    metrics.fps = diagnostics
        .get_measurement(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|measurement| measurement.value.round() as u32);
    metrics.frame_time_ms = diagnostics
        .get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .map(|measurement| measurement.value.round() as u32);
    metrics.entity_count = diagnostics
        .get_measurement(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .map_or(metrics.entity_count, |measurement| {
            measurement.value.round() as usize
        });
    overlay.metrics = *metrics;
}
