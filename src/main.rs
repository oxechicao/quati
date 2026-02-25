use clap::Parser;
use quati::cli::{Actions, Cli, save, start, update};

fn main() {
    let cli = Cli::parse();

    match cli.action {
        Actions::Start {
            branch,
            prefix,
            no_prefix,
            no_push,
        } => {
            start(branch, prefix, no_prefix, no_push, None);
        }
        Actions::Save { context, scope } => {
            let _ = save(context, scope, None);
        }
        Actions::Update { context, scope } => {
            let _ = update(context, scope, None);
        }
    }
}
