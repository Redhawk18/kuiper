use kuiper_gui::start_gui;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    let filter = EnvFilter::from_default_env().add_directive("async_lsp=trace".parse().unwrap());
    fmt().with_env_filter(filter).init();

    let _ = start_gui();
}
