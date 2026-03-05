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
            let _ = start(branch, prefix, no_prefix, no_push, None);
        }
        Actions::Save {
            scope,
            context,
            all,
            emojis,
            no_emojis,
        } => {
            let _ = save(context, scope, all, emojis, no_emojis, None);
        }
        Actions::Update {
            scope,
            context,
            all,
            emojis,
            no_emojis,
        } => {
            let _ = update(context, scope, all, emojis, no_emojis, None);
        }
    }
}
