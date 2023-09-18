use clap::{Arg, ArgAction, Command};

pub fn create_command() -> clap::Command {
    let gui = Command::new("gui")
        .about("Start the gui frontend")
        .long_flag("gui")
        .short_flag('g');

    Command::new("blaze")
        .about("Hybrid IDE")
        .arg_required_else_help(true)
}
