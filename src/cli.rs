use std::path::Path;

use clap::{Parser, Subcommand};
use git2::Error;

use crate::{
    git::{
        create_branch, do_commit, get_current_branch_name, get_repo, git_add_all,
        git_diff_as_string, push_branch,
    },
    logger::Logger,
    prompt,
};

/// CLI to manage git changes with AI assistance
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

/// Action to perform
#[derive(Subcommand)]
pub enum Actions {
    /// Start a new branch locally and remotely
    Start {
        /// Name of the new branch
        branch: Option<String>,

        /// Define some preffix to be used in the branch name, like "feature/" or "bugfix/"
        #[arg(short = 'p', long, default_value = "wip/")]
        prefix: Option<String>,

        #[arg(long = "no-preffix", default_value_t = false)]
        no_prefix: bool,

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
    /// Do the commit with Ai assistance and push the branch to origin
    Update {
        /// Path to the file with some context to help the AI to generate the
        /// commit message
        context: Option<String>,

        /// Scope of the commit message, to be used in the subject
        scope: Option<String>,
    },
}

pub fn start(
    branch: Option<String>,
    prefix: Option<String>,
    no_prefix: bool,
    no_push: bool,
    path: Option<&Path>,
) -> Result<(), Error> {
    Logger.info("executing start");
    let prefix_branc = if no_prefix {
        String::new()
    } else {
        prefix.unwrap()
    };

    let branch_name = format!(
        "{}{}",
        prefix_branc,
        branch.unwrap_or_else(|| "feature/new-branch".to_string()),
    );

    let repo = get_repo(path);

    let current_branch = get_current_branch_name(&repo).unwrap_or_else(|e| {
        eprintln!("Error getting current branch: {}", e);
        String::new()
    });

    if current_branch == branch_name {
        Logger.warn(&format!(
            "Already on branch '{}', skipping branch creation",
            branch_name
        ));
        return Ok(());
    }

    create_branch(&repo, &branch_name)?;

    if no_push {
        return Ok(());
    }

    push_branch(&repo, &branch_name)?;

    Ok(())
}

pub fn save(
    context: Option<String>,
    scope: Option<String>,
    path: Option<&Path>,
) -> Result<(), String> {
    Logger.info("executing save");
    let repo = get_repo(path);
    let _ = git_add_all(&repo);
    let diff = git_diff_as_string(&repo).map_err(|e| format!("Error generating diff: {}", e))?;
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

pub fn update(
    context: Option<String>,
    scope: Option<String>,
    path: Option<&Path>,
) -> Result<(), String> {
    Logger.info("executing update");
    save(context, scope, path)?;
    let repo = get_repo(path);
    let branch_name = get_current_branch_name(&repo)
        .map_err(|e| format!("Error getting current branch name: {}", e))?;
    let result = push_branch(&repo, &branch_name);

    match result {
        Ok(_) => Logger.info(&format!("Branch '{}' pushed successfully", branch_name)),
        Err(e) => Logger.error(format!("Error pushing branch: {}", e).as_str()),
    }

    Ok(())
}
