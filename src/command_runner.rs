use std::process::Command;

#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait CommandRunner {
    fn run(&mut self, program: &str, args: &[&str]) -> std::io::Result<RunResult>;
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

#[cfg(test)]
pub struct MockCommandRunner {
    pub result: RunResult,
}

#[cfg(test)]
impl CommandRunner for MockCommandRunner {
    fn run(&mut self, _program: &str, _args: &[&str]) -> std::io::Result<RunResult> {
        Ok(self.result.clone())
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
        assert_eq!(runner.result.stdout, b"feature/test\n".to_vec());
    }
}
