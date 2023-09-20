mod cli;
use cli::cli;

fn main() {
    pretty_env_logger::init();

    cli();
}
