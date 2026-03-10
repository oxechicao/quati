use clap::Parser;
use quati::cli_commands::{save, start, update};
use quati::cli_signature::{Actions, Cli};

fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.action {
        Actions::Start(args) => {
            let _ = start(args, None);
        }
        Actions::Save(args) => {
            let _ = save(args, None);
        }
        Actions::Update(args) => {
            let _ = update(args, None);
        }
    }
}
