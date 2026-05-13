use alchimera_game::runtime_metrics::run_headless_metrics;

#[test]
fn headless_metrics_runs_requested_frame_count() {
    let report = run_headless_metrics(3);

    assert_eq!(report.frames, 3);
    assert!(report.elapsed_micros > 0);
    assert!(report.entity_count >= report.metrics.entity_count);
}

#[test]
fn headless_metrics_treats_zero_frames_as_one_sample() {
    let report = run_headless_metrics(0);

    assert_eq!(report.frames, 1);
    assert!(report.elapsed_micros > 0);
}
