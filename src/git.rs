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

    pub fn create_branch(&mut self, branch_name: Option<&str>) -> Result<String, String> {
        let branch = match branch_name {
            Some(name) => format!("wip/{}", name.to_string()),
            None => format!("wip/{}", self.get_current_branch_name()?),
        };

        let result = self
            .runner
            .run("git", &["checkout", "-b", &branch])
            .map_err(|e| format!("Failed to create branch: {}", e))?;

        if result.success {
            Ok(branch)
        } else {
            let checkout_only = self
                .runner
                .run("git", &["checkout", &branch])
                .map_err(|e| format!("Failed to checkout branch: {}", e))?;

            if checkout_only.success {
                Ok(branch)
            } else {
                Err(format!(
                    "Error creating branch: {}",
                    String::from_utf8_lossy(&result.stderr)
                ))
            }
        }
    }

    #[cfg(test)]
    pub fn delete_current_branch(&mut self) -> Result<(), String> {
        let current_branch = self.get_current_branch_name()?;
        if current_branch == "main" {
            return Err("Cannot delete the main branch".to_string());
        }

        self.runner.run("git", &["checkout", "main"])
            .map_err(|e| format!("Failed to checkout main branch: {}", e))?;

        let result = self
            .runner
            .run("git", &["branch", "-D", &current_branch])
            .map_err(|e| format!("Failed to delete branch: {}", e))?;

        if result.success {
            Ok(())
        } else {
            Err(format!(
                "Error deleting branch: {}",
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

    #[test]
    fn should_create_default_branch_when_no_branch_name_sent() {
        let mut git = Git::with_mock(RunResult {
            success: true,
            stdout: b"main\n".to_vec(),
            stderr: vec![],
        });
        let result = git.create_branch(None).unwrap();
        assert_eq!(result, "wip/main");
    }

    #[test]
    fn should_return_ok_when_create_branch_with_name() {
        let result = Git::real().create_branch(Some("feature/test"));
        assert!(result.is_ok());
        Git::real().delete_current_branch().unwrap();
    }
}
