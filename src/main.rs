// use kuiper_gui::start_gui;
use kuiper_lsp::start_lsp;
use tokio;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // let _ = start_gui();
    start_lsp().await;
}
