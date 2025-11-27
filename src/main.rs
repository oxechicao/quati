mod args;

use args::Args;
use clap::Parser;

fn main() {
    Args::parse();
}
