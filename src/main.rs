mod args;
mod commands;
mod utils;

use clap::Parser;

fn main() {
    let arg = args::Arg::parse();

    match arg.command {
        args::Commands::Add { key, value } => {
            commands::add::add(key, value);
        }
        args::Commands::Update { key, value } => {
            commands::update::update(key, value);
        }
        args::Commands::Delete { key } => {
            commands::delete::delete(key);
        }
        args::Commands::List { line } => {
            commands::list::list(line);
        }
        args::Commands::Get { key } => {
            commands::get::get(key);
        }
        args::Commands::Clean { force } => {
            commands::clean::clean(force);
        }
        args::Commands::Version {} => {
            commands::version::version();
        }
    }
}
