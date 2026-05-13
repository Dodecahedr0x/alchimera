fn main() {
    let mut args = std::env::args().skip(1);
    let Some(first_arg) = args.next() else {
        alchimera_game::run();
        return;
    };

    match first_arg.as_str() {
        "--help" | "-h" => {
            println!(
                "Alchimera\n\nUSAGE:\n    alchimera [OPTIONS]\n\nOPTIONS:\n    -h, --help                  Print help\n        --headless-metrics [N]  Run N headless scripted traversal frames and print runtime metrics"
            );
        }
        "--headless-metrics" => {
            let frames = args
                .next()
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(60);
            let report = alchimera_game::runtime_metrics::run_headless_traversal_metrics(frames, 7);
            println!(
                "headless_metrics frames={} elapsed_micros={} entity_count={} diagnostics_entity_count={} fps={:?} frame_time_ms={:?} active_chunks={} queued_chunks={} initial_player_chunk=({}, {}) final_player_chunk=({}, {}) unique_player_chunks={}",
                report.frames,
                report.elapsed_micros,
                report.entity_count,
                report.metrics.entity_count,
                report.metrics.fps,
                report.metrics.frame_time_ms,
                report.metrics.active_chunks,
                report.metrics.queued_chunks,
                report.initial_player_chunk.x,
                report.initial_player_chunk.z,
                report.final_player_chunk.x,
                report.final_player_chunk.z,
                report.unique_player_chunks
            );
        }
        _ => alchimera_game::run(),
    }
}
