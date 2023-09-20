use blaze_gui::start_gui;
use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

pub fn create_command() {
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            Command::new("gui")
                .about("Start the gui frontend")
                .long_flag("gui")
                .short_flag('g'),
        )
        .subcommand_required(true)
        .version(crate_version!())
        .get_matches();

    match matches.subcommand() {
        Some(("gui", _)) => {
            let _ = start_gui();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
