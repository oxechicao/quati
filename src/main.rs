mod cli;
mod command_runner;
mod git;
mod logger;
mod prompt;

use clap::Parser;
use cli::{Actions, Cli, save, start, update};

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Actions::Start {
            branch,
            prefix,
            no_prefix,
            no_push,
        } => {
            start(branch, prefix, no_prefix, no_push);
        }
        Actions::Save { context, scope } => {
            let _ = save(context, scope);
        }
        Actions::Update { context, scope } => {
            let _ = update(context, scope);
        }
    }
}
