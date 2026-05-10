fn main() {
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        println!("Alchimera\n\nUSAGE:\n    alchimera [OPTIONS]\n\nOPTIONS:\n    -h, --help    Print help");
        return;
    }

    alchimera_game::run();
}
