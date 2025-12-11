// mod cli_struct;
// mod git;
// mod start;

// use clap::Parser;
// use cli_struct::{Actions, Cli};
// use git::get_current_branch_name;
// use start::run_start;

fn main() {
    println!("This is a placeholder for the main function.");
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


pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub struct MockCommandRunner {
    pub result: RunResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_mock_command_runner_instance_correctly() {
        let runner = MockCommandRunner {
            result: RunResult {
                success: true,
                stdout: b"feature/test\n".to_vec(),
                stderr: vec![],
            },
        };
        assert_eq!(
            runner.result.stdout,
            b"feature/test\n".to_vec()
        );
    }
}

