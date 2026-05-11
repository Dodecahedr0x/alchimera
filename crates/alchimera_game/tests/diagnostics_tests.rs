use alchimera_game::{
    build_app,
    diagnostics::{DiagnosticsOverlay, RuntimeDiagnostics, ToggleDiagnosticsOverlay},
};
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};

#[test]
fn diagnostics_plugin_inserts_metrics_resource() {
    let app = build_app();
    let overlay = app.world().resource::<DiagnosticsOverlay>();
    let metrics = app.world().resource::<RuntimeDiagnostics>();

    assert!(!overlay.visible);
    assert_eq!(*metrics, RuntimeDiagnostics::default());
    assert_eq!(overlay.metrics, RuntimeDiagnostics::default());
}

#[test]
fn diagnostics_plugin_registers_bevy_metric_paths() {
    let app = build_app();
    let diagnostics = app.world().resource::<DiagnosticsStore>();

    assert!(diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).is_some());
    assert!(diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .is_some());
    assert!(diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .is_some());
}

#[test]
fn toggle_event_changes_overlay_visibility() {
    let mut app = build_app();

    app.world_mut().send_event(ToggleDiagnosticsOverlay);
    app.update();

    assert!(app.world().resource::<DiagnosticsOverlay>().visible);

    app.world_mut().send_event(ToggleDiagnosticsOverlay);
    app.update();

    assert!(!app.world().resource::<DiagnosticsOverlay>().visible);
}

#[test]
fn every_toggle_event_is_applied() {
    let mut app = build_app();

    app.world_mut().send_event(ToggleDiagnosticsOverlay);
    app.world_mut().send_event(ToggleDiagnosticsOverlay);
    app.update();

    assert!(!app.world().resource::<DiagnosticsOverlay>().visible);
}
