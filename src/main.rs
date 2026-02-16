mod cli;
mod command_runner;
mod git;
// mod start;

use clap::Parser;
// use clap::Parser;
// use cli_struct::{Actions, Cli};
use cli::{Actions, Cli};
use git::Git;
// use start::run_start;

fn main() {
    println!("This is a placeholder for the main function.");
    match Git::real().get_current_branch_name() {
        Ok(branch) => println!("Current branch: {}", branch),
        Err(e) => eprintln!("Error: {}", e),
    }

    let cli = Cli::parse();
    match cli.action {
        Actions::Start {
            branch,
            skip_hooks,
            remove_prefix,
            no_push,
        } => {
            Git::real().create_branch(branch.as_deref());
        }
        Actions::Save { context } => {
            println!("Saving with context: {:?}", context);
        }
    }
}
