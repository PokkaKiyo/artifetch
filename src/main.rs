use features::{add::do_add, fetch::do_fetch, remove::do_remove, show::do_show};

mod artifact;
mod cli;
mod config;
mod features;
mod utils;

fn main() {
    match cli::cli().get_matches().subcommand() {
        Some(("add", submatches)) => do_add(
            submatches.get_one::<String>("name").unwrap(),
            submatches.get_one::<String>("url").unwrap(),
            submatches.get_one::<String>("sha256sum").unwrap(),
        ),
        Some(("remove", submatches)) => do_remove(
            submatches.get_one::<String>("name").unwrap(),
            submatches.get_one::<bool>("force").unwrap(),
        ),
        Some(("show", _submatches)) => do_show(),
        Some(("fetch", _submatches)) => do_fetch(),
        None => panic!("No subcommand?"),
        _ => panic!("Unimplemented subcommand?"),
    }
}
