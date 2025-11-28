mod cli_struct;

use clap::Parser;
use cli_struct::Cli;

fn main() {
    Cli::parse();
}
