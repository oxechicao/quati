use crate::command_runner::{CommandRunner};

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
    use crate::command_runner::{MockCommandRunner, RunResult, RealCommandRunner};

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

    #[test]
    fn test_with_real_runner() {
        let mut runner = RealCommandRunner;
        let result = get_current_branch_name(&mut runner);
        assert!(result.is_ok());
    }

}

