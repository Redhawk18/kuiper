use blaze_gui::start_gui;
use clap::Command;

pub fn create_command() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about("Hybrid integrated development environment")
        .subcommand_required(true)
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("gui")
                .about("Start the gui frontend")
                .long_flag("gui")
                .short_flag('g'),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("gui", _)) => {
            let _ = start_gui();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
