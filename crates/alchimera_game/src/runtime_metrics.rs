//! Headless runtime metrics collection for cron/CI environments.

use std::time::Instant;

use crate::{build_app, diagnostics::RuntimeDiagnostics};

/// Summary produced by a bounded headless app-update run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadlessMetricsReport {
    /// Number of Bevy update frames executed.
    pub frames: u32,
    /// Wall-clock time spent running those frames, in microseconds.
    pub elapsed_micros: u128,
    /// Latest diagnostics resource sampled from the app.
    pub metrics: RuntimeDiagnostics,
    /// Live entity count observed directly from the Bevy world after the run.
    pub entity_count: usize,
}

/// Runs the minimal game app for a bounded number of update frames without opening a window.
#[must_use]
pub fn run_headless_metrics(requested_frames: u32) -> HeadlessMetricsReport {
    let frames = requested_frames.max(1);
    let mut app = build_app();

    let start = Instant::now();
    for _ in 0..frames {
        app.update();
    }
    let elapsed_micros = start.elapsed().as_micros().max(1);

    let metrics = *app.world().resource::<RuntimeDiagnostics>();
    let entity_count = app.world().entities().len() as usize;

    HeadlessMetricsReport {
        frames,
        elapsed_micros,
        metrics,
        entity_count,
    }
}
