use kuiper_gui::start_gui;
use tracing_subscriber::{EnvFilter, fmt};

fn main() {
    let filter = EnvFilter::from_default_env()
        .add_directive("kuiper_gui=trace".parse().unwrap())
        .add_directive("kuiper_lsp=trace".parse().unwrap());
    fmt().with_env_filter(filter).init();

    let _ = start_gui();
}
