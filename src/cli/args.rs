use clap::ValueEnum;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anvl", version, about)]
pub struct Args {
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init {
        kind: InitKind,
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
        check: DoctorCheck,
    },
    List,
    Info {
        module: String,
    },
    Cache {
        action: CacheAction,
    },
    Config {
        #[command(subcommand)]
        cmd: Option<ConfigCommand>,
    },
    Create {
        kind: CreateKind,
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

#[derive(ValueEnum, Clone, Debug)]
pub enum InitKind {
    Lib,
    Bin,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CreateKind {
    C,
    H,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum DoctorCheck {
    Template,
    Project,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    Get,
    Set { key: String, value: String },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CacheAction {
    Update,
    Clean,
}
