use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anvl", version, about)]
pub struct Args {
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        #[command(subcommand)]
        kind: InitKindCommand,
        name: String,
        #[arg(short = 'p', long)]
        push: bool,
    },
    Install {
        modules: Vec<String>,
        #[arg(short = 'p', long)]
        push: bool,
    },
    Update {
        modules: Vec<String>,
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'p', long)]
        push: bool,
    },
    Remove {
        modules: Vec<String>,
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'p', long)]
        push: bool,
    },
    Status,
    Doctor {
        #[command(subcommand)]
        cmd: DoctorCommand,
    },
    List,
    Info {
        module: String,
    },
    Cache {
        #[command(subcommand)]
        cmd: CacheCommand,
    },
    Config {
        #[command(subcommand)]
        cmd: Option<ConfigCommand>,
    },
    Create {
        #[command(subcommand)]
        kind: CreateKindCommand,
        path: String,
        #[arg(short = 'p', long)]
        push: bool,
        #[arg(long = "no-test")]
        no_test: bool,
    },
    Delete {
        path: String,
        #[arg(short = 'p', long)]
        push: bool,
    },
}

#[derive(Subcommand)]
pub enum InitKindCommand {
    Lib,
    Bin,
}

#[derive(Subcommand)]
pub enum CreateKindCommand {
    C,
    H,
}

#[derive(Subcommand)]
pub enum DoctorCommand {
    Template,
    Project,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Get,
    Set { key: String, value: String },
}

#[derive(Subcommand)]
pub enum CacheCommand {
    Update,
    Clean,
}
