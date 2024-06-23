use clap::{Arg, ArgAction, Command};

use crate::features::{
    add::subcmd_add, fetch::subcmd_fetch, remove::subcmd_remove, show::subcmd_show,
};

pub fn cli() -> Command {
    Command::new("artifetch")
        .version("0.1.0")
        .about("Artifetch: fetch online artifacts")
        .arg(arg_verbosity())
        .subcommand_required(true)
        .subcommand(subcmd_add())
        .subcommand(subcmd_remove())
        .subcommand(subcmd_show())
        .subcommand(subcmd_fetch())
}

fn arg_verbosity() -> Arg {
    Arg::new("verbosity")
        .long("verbose")
        .short('v')
        .help("Set the verbosity of the program")
        .action(ArgAction::Count)
        .required(false)
}
