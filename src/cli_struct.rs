use clap::{Parser, Subcommand};

/// CLI to manage git changes with AI assistance
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    action: Actions,

    // Run in quiet mode
    #[arg(short = 'q', long)]
    quiet: Option<bool>,
}

/// Action to perform
#[derive(Subcommand)]
enum Actions {
    /// Start a new branch locally and remotely
    Start {
        /// Name of the new branch
        branch: String,

        /// Skip git hooks
        #[arg(short = 'N', long = "no-verify", default_value_t = false)]
        skip_hooks: bool,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        crate::cli_struct::Cli::command().debug_assert();
    }
}
