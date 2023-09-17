use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Arg {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    Update {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    Delete {
        #[arg(short, long)]
        key: String,
    },
    List {
        #[arg(short, long)]
        line: bool,
    },
    Get {
        #[arg(short, long)]
        key: String,
    },
    Clean {
        #[arg(short, long)]
        force: bool,
    },
}
