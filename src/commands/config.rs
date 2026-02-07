use crate::cli::args::ConfigCommand;
use crate::core::config::Config;

pub fn dispatch(cmd: &ConfigCommand) -> Result<(), String> {
    match cmd {
        ConfigCommand::Get => cmd_get(),
        ConfigCommand::Set { key, value } => cmd_set(key, value),
    }
}

fn cmd_get() -> Result<(), String> {
    let cfg = Config::get()?;

    println!("Anvl configuration:\n");
    println!("  repo -> {}", cfg.repo);
    println!("  anvl_storage_path -> {}", cfg.anvl_storage_path.display());
    println!("  always_push -> {}", cfg.always_push);
    Ok(())
}

fn cmd_set(key: &str, value: &str) -> Result<(), String> {
    Config::set(key.to_string(), value.to_string())?;
    println!("config updated: {key} -> {value}");
    Ok(())
}
