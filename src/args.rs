use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct Arg {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
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
        #[arg(short = 'k', long, help = "Show only keys without values")]
        keys_only: bool,
    },
    Get {
        key: String,
    },
    Clean {
        #[arg(short, long)]
        force: bool,
    },
    Version,
}
