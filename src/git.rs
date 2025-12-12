#[cfg(test)]
use crate::command_runner::{MockCommandRunner, RunResult};
use crate::command_runner::{CommandRunner, RealCommandRunner};

pub struct Git<R: CommandRunner> {
    runner: R,
}

impl<R: CommandRunner> Git<R> {
    pub fn get_current_branch_name(&mut self) -> Result<String, String> {
        let result = self
            .runner
            .run("git", &["rev-parse", "--abbrev-ref", "HEAD"])
            .map_err(|e| format!("Failed: {}", e))?;

        if result.success {
            Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
        } else {
            Err(format!(
                "Error: {}",
                String::from_utf8_lossy(&result.stderr)
            ))
        }
    }
}

impl Git<RealCommandRunner> {
    pub fn real() -> Self {
        Self {
            runner: RealCommandRunner,
        }
    }
}

#[cfg(test)]
impl Git<MockCommandRunner> {
    pub fn with_mock(result: RunResult) -> Self {
        Self {
            runner: MockCommandRunner { result },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_runner::{RunResult};

    #[test]
    fn should_return_branch_name() {
        let mut git = Git::with_mock(RunResult {
            success: true,
            stdout: b"feature/test\n".to_vec(),
            stderr: vec![],
        });

        let result = git.get_current_branch_name().unwrap();
        assert_eq!(result, "feature/test");
    }

    #[test]
    fn test_with_real_runner() {
        let result = Git::real().get_current_branch_name();
        assert!(result.is_ok());
    }
}
