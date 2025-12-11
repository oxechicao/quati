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


#[derive(Clone,Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}


pub trait CommandRunner {
    fn run(&mut self, program: &str, args: &[&str]) -> std::io::Result<RunResult>;
}

pub struct MockCommandRunner {
    pub result: RunResult,
}

impl CommandRunner for MockCommandRunner {
    fn run(&mut self, _program: &str, _args: &[&str]) -> std::io::Result<RunResult> {
        Ok(self.result.clone())
    }
}

pub struct RealCommandRunner;
impl CommandRunner for RealCommandRunner {
    fn run(&mut self, program: &str, args: &[&str]) -> std::io::Result<RunResult> {
        let output = Command::new(program).args(args).output()?;
        Ok(RunResult {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

pub fn get_current_branch_name<R: CommandRunner>(runner: &mut R) -> Result<String, String> {
   let result = runner.run("git", &["rev-parse", "--abbrev-ref", "HEAD"])
        .map_err(|e| format!("Failed: {}", e))?;

    if result.success {
        Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
    } else {
        Err(format!("Error: {}", String::from_utf8_lossy(&result.stderr)))
    }
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

    #[test]
    fn should_return_branch_name() {
        let mut runner = MockCommandRunner {
            result: RunResult {
                success: true,
                stdout: b"feature/test\n".to_vec(),
                stderr: vec![],
            },
        };

        let result = get_current_branch_name(&mut runner).unwrap();
        assert_eq!(result, "feature/test");
    }

}

