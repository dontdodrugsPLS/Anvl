mod cache;

use crate::cli::args::{Args, Command};

pub fn run(args: Args) -> Result<(), String> {
    match args.command {
        Command::Cache { action } => cache::dispatch(action, args.verbose)?,
        _ => {
            println!("command {:?} is not implemented yet!", args.command)
        }
    }
    Ok(())
}
