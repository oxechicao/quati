use clap::{Parser, Subcommand};

use crate::{file, git::Git, logger::Logger, prompt};

/// CLI to manage git changes with AI assistance
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,

    /// Run in quiet mode
    #[arg(short = 'q', long)]
    pub quiet: Option<bool>,
}

/// Action to perform
#[derive(Subcommand)]
pub enum Actions {
    /// Start a new branch locally and remotely
    Start {
        /// Name of the new branch
        branch: Option<String>,

        /// Skip git hooks
        #[arg(short = 'N', long = "no-verify", default_value_t = false)]
        skip_hooks: bool,

        /// Remove the prefix from the branch name
        #[arg(short = 'P', long = "no-prefix", default_value_t = false)]
        remove_prefix: bool,

        /// Do not push the branch to origin
        #[arg(short = 'S', long = "no-push", default_value_t = false)]
        no_push: bool,
    },
    /// Action to do the commit with AI Assistance
    Save {
        /// Path to the file with some context to help the AI to generate the
        /// commit message
        context: Option<String>,

        /// Scope of the commit message, to be used in the subject
        scope: Option<String>,
    },
}

pub fn save(context: Option<String>, scope: Option<String>) {
    Logger.info("executing save");
    let diff = Git::real().get_diff().unwrap_or_else(|e| {
        Logger.error(&format!("Error getting diff: {}", e));
        String::new()
    });

    match file::write_file("temp_git_changes.diff", &diff) {
        Ok(_) => {
            Git::real()
                .add_all()
                .unwrap_or_else(|e| Logger.error(&format!("Error adding file to git: {}", e)));

            Logger.info("created diff file");
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
                .unwrap_or_else(|e| Logger.error(&format!("Error committing changes: {}", e)));

            let _ = file::rm_file("temp_git_changes.diff");
        }
        Err(e) => Logger.error(&format!("Errmr writing file: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        crate::cli::Cli::command().debug_assert();
    }
}
