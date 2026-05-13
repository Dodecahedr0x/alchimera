use alchimera_game::runtime_metrics::{run_headless_metrics, run_headless_traversal_metrics};
use alchimera_generation::chunk::ChunkCoord;

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

#[test]
fn headless_traversal_metrics_move_player_across_streamed_chunks() {
    let report = run_headless_traversal_metrics(4, 7);

    assert_eq!(report.frames, 4);
    assert_eq!(report.initial_player_chunk, ChunkCoord::new(0, 0));
    assert_ne!(report.final_player_chunk, report.initial_player_chunk);
    assert!(report.unique_player_chunks >= 2);
    assert!(report.metrics.queued_chunks >= 9);
}
