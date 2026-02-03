use crate::cli::args::{Args, Command};

pub fn run(args: Args) -> Result<(), String> {
    match args.command {
        Command::Init { .. } => {}
        _ => {
            println!("command {:?} is not implemented yet!", args.command)
        }
    }
    Ok(())
}
