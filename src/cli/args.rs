use clap::ValueEnum;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anvl", version, about)]
pub struct Args {
    #[arg(short = 'v', long, help = "Make Anvl verbose")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Initialize an Anvl project inside an empty repository")]
    Init {
        #[arg(help = "Type of deliverable")]
        kind: InitKind,
        #[arg(help = "Project name")]
        name: String,
        #[arg(short = 'p', long, help = "Automatically commit and push changes")]
        push: bool,
    },
    #[command(about = "Install one ore more modules from cache into the current project")]
    Install {
        #[arg(help = "Modules list (e.g. vec io str)")]
        modules: Vec<String>,
        #[arg(short = 'p', long, help = "Automatically commit and push changes")]
        push: bool,
    },
    #[command(about = "Update one or more modules from cache into the current project")]
    Update {
        #[arg(help = "Modules list (e.g. vec io str)")]
        modules: Vec<String>,
        #[arg(short = 'f', long, help = "Force update locally modified modules")]
        force: bool,
        #[arg(short = 'p', long, help = "Automatically commit and push changes")]
        push: bool,
    },
    #[command(about = "Remove one or more modules installed into the current project")]
    Remove {
        #[arg(help = "Modules list (e.g. vec io str)")]
        modules: Vec<String>,
        #[arg(
            short = 'f',
            long,
            help = "Force delete modules that are required by other modules installed (this will surely broke your project)"
        )]
        force: bool,
        #[arg(short = 'p', long, help = "Automatically commit and push changes")]
        push: bool,
    },
    #[command(about = "Track modules state across the current project")]
    Status,
    #[command(
        about = "Verify if the current project or template repository is aligned with Anvl architecture"
    )]
    Doctor { check: DoctorCheck },
    #[command(about = "List every available module inside cache")]
    List,
    #[command(about = "Get informations about a module")]
    Info { module: String },
    #[command(about = "Update or clean local Anvl module and template cache")]
    Cache { action: CacheAction },
    #[command(about = "Customize and configure Anvl tool")]
    Config {
        #[command(subcommand)]
        cmd: ConfigCommand,
    },
    #[command(about = "Create a new file inside the current project")]
    Create {
        kind: CreateKind,
        path: String,
        #[arg(short = 'p', long)]
        push: bool,
        #[arg(long = "no-test")]
        no_test: bool,
    },
    #[command(about = "Delete a file inside the current project")]
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
    #[command(about = "Print the current Anvl configuration")]
    Get,
    #[command(about = "Change an Anvl configuration field")]
    Set { key: String, value: String },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CacheAction {
    Update,
    Clean,
}
