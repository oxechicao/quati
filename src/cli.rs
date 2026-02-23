use clap::{Parser, Subcommand};
use git2::{DiffOptions, Repository};

use crate::{
    git::{do_commit, git_add_all, git_diff_as_string},
    logger::Logger,
    prompt,
};

pub fn save(context: Option<String>, scope: Option<String>) -> Result<(), String> {
    Logger.info("executing save");
    let repo = Repository::open("./").unwrap_or_else(|e| {
        eprintln!("Error opening repository: {}", e);
        std::process::exit(1);
    });

    let _ = git_add_all(&repo);

    let mut opts = DiffOptions::new();
    let repo_diff = repo
        .diff_index_to_workdir(None, Some(&mut opts))
        .map_err(|e| format!("Failed to add files: {}", e))?;

    let diff =
        git_diff_as_string(&repo_diff).map_err(|e| format!("Error generating diff: {}", e))?;

    let message = prompt::generate_commit_message(
        context.as_deref().unwrap_or(""),
        scope.as_deref().unwrap_or(""),
        &diff,
    )
    .unwrap_or_else(|e| {
        eprintln!("Error generating commit message: {}", e);
        String::new()
    });

    let _ = do_commit(&repo, &message);

    Ok(())
}

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

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        crate::cli::Cli::command().debug_assert();
    }
}
