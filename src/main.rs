mod cli;
mod command_runner;
mod file;
mod git;
mod logger;
mod prompt;

use clap::Parser;
use cli::{Actions, Cli, save};
use git::Git;

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Actions::Start {
            branch,
            skip_hooks,
            remove_prefix,
            no_push,
        } => {
            println!(
                "Starting a new branch with options: skip_hooks={}, remove_prefix={}, no_push={}",
                skip_hooks, remove_prefix, no_push
            );
            let _ = Git::real().create_branch(branch.as_deref());
        }
        Actions::Save { context, scope } => save(context, scope),
    }
}
