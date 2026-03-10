use clap::{Args, Parser, Subcommand};

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
    Start(StartArgs),
    /// Action to do the commit with AI Assistance
    Save(SaveArgs),
    /// Do the commit with Ai assistance and push the branch to origin
    Update(UpdateArgs),
}

#[derive(Args)]
pub struct StartArgs {
    /// Name of the new branch
    pub branch: Option<String>,

    /// Define some preffix to be used in the branch name, like "feature/" or "bugfix/"
    #[arg(short = 'p', long, default_value = "wip/")]
    pub prefix: Option<String>,

    #[arg(long = "no-preffix", default_value_t = false)]
    pub no_prefix: bool,

    /// Do not push the branch to origin
    #[arg(short = 'S', long = "no-push", default_value_t = false)]
    pub no_push: bool,
}

#[derive(Args)]
pub struct SaveArgs {
    /// Scope of the commit message, to be used in the subject
    pub scope: Option<String>,

    /// Path to the file with some context to help the AI to generate the
    /// commit message
    #[arg(short, long = "context")]
    pub context: Option<String>,

    /// All all changes to commit, including untracked files
    #[arg(short, long = "all", default_value_t = false)]
    pub all: bool,

    /// Use emojis in the commit message
    #[arg(short, long = "emojis", default_value_t = false)]
    pub emojis: bool,

    /// Do not use emojis in the commit message
    #[arg(short, long = "no-emojis", default_value_t = false)]
    pub no_emojis: bool,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Scope of the commit message, to be used in the subject
    pub scope: Option<String>,

    /// Path to the file with some context to help the AI to generate the
    /// commit message
    #[arg(short, long = "context")]
    pub context: Option<String>,

    /// All all changes to commit, including untracked files
    #[arg(short, long = "all", default_value_t = false)]
    pub all: bool,

    /// Use emojis in the commit message
    #[arg(short, long = "emojis", default_value_t = false)]
    pub emojis: bool,

    /// Do not use emojis in the commit message
    #[arg(short, long = "no-emojis", default_value_t = false)]
    pub no_emojis: bool,
}
