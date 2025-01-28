use kuiper_gui::start_gui;

fn main() {
    tracing_subscriber::fmt::init();
    // pretty_env_logger::init();

    let _ = start_gui();
}
