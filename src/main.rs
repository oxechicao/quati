mod cli;
mod command_runner;
mod file;
mod git;
mod prompt;

use clap::Parser;
use cli::{Actions, Cli};
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
        Actions::Save { context, scope } => {
            println!("executing save");
            let diff = Git::real().get_diff().unwrap_or_else(|e| {
                eprintln!("Error getting diff: {}", e);
                String::new()
            });

            match file::write_file("temp_git_changes.diff", &diff) {
                Ok(_) => {
                    Git::real()
                        .add_all()
                        .unwrap_or_else(|e| eprintln!("Error adding file to git: {}", e));

                    println!("created diff file");
                    let message = prompt::generate_commit_message(
                        context.as_deref().unwrap_or(""),
                        scope.as_deref().unwrap_or(""),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("Error generating commit message: {}", e);
                        String::new()
                    });

                    Git::real()
                        .commit_changes(&message)
                        .unwrap_or_else(|e| eprintln!("Error committing changes: {}", e));

                    let _ = file::rm_file("temp_git_changes.diff");
                }
                Err(e) => println!("Error writing file: {}", e),
            }

            println!("Saving with context: {:?}", context);
        }
    }
}
