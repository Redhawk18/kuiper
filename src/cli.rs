use blaze_gui::start_gui;
use blaze_tui::start_tui;
use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

pub fn cli() {
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            Command::new("gui")
                .about("Start the gui frontend")
                .long_flag("gui")
                .short_flag('g'),
        )
        .subcommand(
            Command::new("tui")
                .about("Start the tui frontend")
                .long_flag("tui")
                .short_flag('t'),
        )
        .subcommand_required(true)
        .version(crate_version!())
        .get_matches();

    match matches.subcommand() {
        Some(("gui", _)) => {
            let _ = start_gui();
        }

        Some(("tui", _)) => {
            start_tui();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
