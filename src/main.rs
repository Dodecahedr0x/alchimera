fn main() {
    let mut args = std::env::args().skip(1);
    let Some(first_arg) = args.next() else {
        alchimera_game::run();
        return;
    };

    match first_arg.as_str() {
        "--help" | "-h" => {
            println!(
                "Alchimera\n\nUSAGE:\n    alchimera [OPTIONS]\n\nOPTIONS:\n    -h, --help                  Print help\n        --headless-metrics [N]  Run N headless update frames and print runtime metrics"
            );
        }
        "--headless-metrics" => {
            let frames = args
                .next()
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(60);
            let report = alchimera_game::runtime_metrics::run_headless_metrics(frames);
            println!(
                "headless_metrics frames={} elapsed_micros={} entity_count={} diagnostics_entity_count={} fps={:?} frame_time_ms={:?} active_chunks={} queued_chunks={}",
                report.frames,
                report.elapsed_micros,
                report.entity_count,
                report.metrics.entity_count,
                report.metrics.fps,
                report.metrics.frame_time_ms,
                report.metrics.active_chunks,
                report.metrics.queued_chunks
            );
        }
        _ => alchimera_game::run(),
    }
}
