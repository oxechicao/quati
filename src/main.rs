// mod cli_struct;
mod command_runner;
mod git;
// mod start;

// use clap::Parser;
// use cli_struct::{Actions, Cli};
use command_runner::{RealCommandRunner};
use git::{get_current_branch_name};
// use start::run_start;


fn main() {
    println!("This is a placeholder for the main function.");
    let mut runner = RealCommandRunner;
    match get_current_branch_name(&mut runner) {
        Ok(branch) => println!("Current branch: {}", branch),
        Err(e) => eprintln!("Error: {}", e),
    }
    // let cli = Cli::parse();

    // match cli.action {
    //     Actions::Start {
    //         branch,
    //         skip_hooks,
    //         remove_prefix,
    //         no_push,
    //     } => {
    //         run_start(
    //             branch
    //                 .as_deref()
    //                 .unwrap_or(get_current_branch_name().as_str())
    //                 .to_string(),
    //             skip_hooks,
    //             remove_prefix,
    //             no_push,
    //         );
    //     }
    // }
}

