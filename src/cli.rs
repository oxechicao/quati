use clap::{Parser, Subcommand};

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
