use git2::{Diff, DiffFormat, Error, IndexAddOption, Oid, Repository};
use std::error::Error as StdError;

use crate::command_runner::{CommandRunner, RealCommandRunner};
#[cfg(test)]
use crate::command_runner::{MockCommandRunner, RunResult};

pub fn git_add_all(repo: &Repository) -> Result<(), Box<dyn StdError>> {
    let mut index = repo.index()?;
    index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    Ok(())
}

pub fn git_diff_as_string(diff: &Diff) -> Result<String, Error> {
    let mut buf = String::new();

    // The print method iterates over the diff and calls the closure for each line.
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        // Line origin is a single character indicating the type of line ('+', '-', ' ', etc.)
        let origin = line.origin();
        if origin != '>' && origin != '<' && origin != 'F' && origin != 'H' {
            // Append the origin character
            buf.push(origin);
        }

        // Convert the content to a string slice and append it
        let content = str::from_utf8(line.content()).unwrap_or_default();
        buf.push_str(content);

        // Return true to continue the iteration
        true
    })?;

    Ok(buf)
}

pub fn do_commit(repo: &Repository, message: &str) -> Result<Oid, git2::Error> {
    // 1. Preparar o Index (git add .)
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // 2. Gravar a Tree (snapshot do estado atual)
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // 3. Definir o Autor e Committer
    let signature = repo.signature()?; // Usa configs globais do git

    // 4. Obter o Commit Pai (se existir)
    let mut parents = Vec::new();
    if let Ok(head) = repo.head() {
        parents.push(head.peel_to_commit()?);
    }

    // 5. Criar o Commit
    let parents_refs: Vec<&git2::Commit> = parents.iter().collect();
    let commit_oid = repo.commit(
        Some("HEAD"),  // Atualiza o HEAD para este novo commit
        &signature,    // Autor
        &signature,    // Committer
        message,       // Mensagem
        &tree,         // Árvore de arquivos
        &parents_refs, // Lista de pais
    )?;

    Ok(commit_oid)
}

pub struct Git<R: CommandRunner> {
    runner: R,
}

impl<R: CommandRunner> Git<R> {
    pub fn add_all(&mut self) -> Result<(), String> {
        let result = self
            .runner
            .run("git", &["add", "."])
            .map_err(|e| format!("Failed to add files: {}", e))?;

        if result.success {
            Ok(())
        } else {
            Err(format!(
                "Error adding files: {}",
                String::from_utf8_lossy(&result.stderr)
            ))
        }
    }

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

    pub fn get_diff(&mut self) -> Result<String, String> {
        let result = self
            .runner
            .run("git", &["diff"])
            .map_err(|e| format!("Failed to get diff: {}", e))?;

        if result.success {
            Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
        } else {
            Err(format!(
                "Error getting diff: {}",
                String::from_utf8_lossy(&result.stderr)
            ))
        }
    }

    pub fn commit_changes(&mut self, message: &str) -> Result<(), String> {
        let result = self
            .runner
            .run("git", &["commit", "-am", message])
            .map_err(|e| format!("Failed to commit changes: {}", e))?;

        if result.success {
            Ok(())
        } else {
            Err(format!(
                "Error committing changes: {}",
                String::from_utf8_lossy(&result.stderr)
            ))
        }
    }

    #[cfg(test)]
    pub fn delete_current_branch(&mut self) -> Result<(), String> {
        let current_branch = self.get_current_branch_name()?;
        if current_branch == "main" {
            return Err("Cannot delete the main branch".to_string());
        }

        self.runner
            .run("git", &["checkout", "main"])
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
    use crate::command_runner::RunResult;

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
    fn should_return_diff() {
        let mut git = Git::with_mock(RunResult {
            success: true,
            stdout: b"diff --git a/file.txt b/file.txt\n".to_vec(),
            stderr: vec![],
        });

        let result = git.get_diff().unwrap();
        assert_eq!(result, "diff --git a/file.txt b/file.txt");
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
