use git2::{
    Cred, Diff, DiffFormat, DiffOptions, Error, IndexAddOption, Oid, PushOptions, RemoteCallbacks,
    Repository,
};
use std::{error::Error as StdError, path::Path};

use crate::{environments::Env, logger::Logger};

pub fn get_repo(path: Option<&Path>) -> Repository {
    let target = path.unwrap_or_else(|| Path::new("./"));
    Repository::open(target).unwrap_or_else(|e| {
        Logger.error(&format!(
            "Error opening repository at '{}': {}",
            target.display(),
            e
        ));
        std::process::exit(1);
    })
}

pub fn get_diff(repo: &Repository) -> Result<Diff<'_>, Error> {
    let mut opts = DiffOptions::new();
    opts.include_untracked(true);
    opts.ignore_whitespace(true);
    opts.ignore_blank_lines(true);
    let head_tree = repo
        .head()
        .and_then(|head| head.resolve())
        .and_then(|reference| reference.peel_to_tree())
        .ok();
    let index = repo.index()?;
    let repo_diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), Some(&mut opts))?;
    Ok(repo_diff)
}

pub fn git_add_all(repo: &Repository) -> Result<(), Box<dyn StdError>> {
    let mut index = repo.index()?;
    index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn git_diff_as_string(repo: &Repository) -> Result<String, Error> {
    let diff = get_diff(repo)?;
    let mut buf = String::new();

    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let origin = line.origin();
        if origin != '>' && origin != '<' && origin != 'F' && origin != 'H' {
            buf.push(origin);
        }

        let content = str::from_utf8(line.content()).unwrap_or_default();
        buf.push_str(content);
        // Return true to continue the iteration
        true
    })?;

    Ok(buf)
}

pub fn do_commit(repo: &Repository, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;
    let mut parents = Vec::new();
    if let Ok(head) = repo.head() {
        parents.push(head.peel_to_commit()?);
    }

    let parents_refs: Vec<&git2::Commit> = parents.iter().collect();
    let commit_oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parents_refs,
    )?;

    Ok(commit_oid)
}

pub fn create_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let head = repo.head()?;
    let target = head
        .target()
        .ok_or_else(|| git2::Error::from_str("Failed to get target OID from HEAD"))?;

    repo.branch(branch_name, &repo.find_commit(target)?, false)?;

    Ok(())
}

pub fn push_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    Logger.info(&format!(
        "Pushing branch '{}' to remote 'origin'",
        branch_name
    ));

    let mut remote = repo.find_remote("origin")?;
    let url = remote.url().unwrap_or("");
    let local_host = Env::get_custom_local_git_host().unwrap_or_else(|e| {
        Logger.info(format!("Custom local git host not set: {}", e).as_str());
        Logger.info(format!("Using default remote URL: {}", url).as_str());
        "".to_string()
    });

    let remote_host = Env::get_custom_remote_git_host().unwrap_or_else(|e| {
        Logger.info(format!("Custom remote git host not set: {}", e).as_str());
        Logger.info(format!("Using default remote URL: {}", url).as_str());
        "github.com:".to_string()
    });
    if url.contains(local_host.as_str()) {
        let real_url = url.replace(local_host.as_str(), remote_host.as_str());
        remote = repo.remote_anonymous(&real_url)?;
    }

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, user, _types| {
        let username = user.unwrap_or("git");
        let key_path = &Env::get_ssh_key_path();
        Cred::ssh_key(username, None, key_path, None)
    });

    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], Some(&mut options))?;
    Ok(())
}

pub fn get_current_branch_name(repo: &Repository) -> Result<String, git2::Error> {
    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or_else(|| git2::Error::from_str("Failed to get branch name from HEAD"))?;
    Ok(branch_name.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;
    use git2::Repository;
    use serial_test::serial;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, Repository) {
        let td = TempDir::new().unwrap();
        {
            let repo = Repository::init(td.path()).unwrap();

            // We need an initial commit to have a HEAD to branch from
            let signature = repo.signature().unwrap();
            let mut index = repo.index().unwrap();
            let tree_id = index.write_tree().unwrap();
            let tree = repo.find_tree(tree_id).unwrap();

            let oid = repo
                .commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    "Initial commit",
                    &tree,
                    &[],
                )
                .unwrap();
            let commit = repo.find_commit(oid).unwrap();
            repo.branch("main", &commit, false).unwrap();
            repo.set_head("refs/heads/main").unwrap();
        }

        let repo = Repository::init(td.path()).unwrap();
        (td, repo)
    }

    #[test]
    fn test_get_repo_success() {
        let (td, _) = setup_test_repo();
        let repo = get_repo(Some(td.path()));
        let repo_path = repo.path().canonicalize().unwrap();
        let expected_path = td.path().join(".git").canonicalize().unwrap();
        assert_eq!(repo_path, expected_path);
    }

    #[test]
    fn test_get_repo_to_current_dir() {
        let repo = get_repo(None);
        let cwd = std::env::current_dir().unwrap().canonicalize().unwrap();
        let repo_path = repo.path().canonicalize().unwrap();
        assert_eq!(repo_path, cwd.join(".git").canonicalize().unwrap());
    }

    #[test]
    fn test_create_branch_success() {
        let (_td, repo) = setup_test_repo();
        let branch_name = "feature-xyz";

        // Call your function
        let result = create_branch(&repo, branch_name);

        // Assert
        assert!(result.is_ok());
        let branch = repo.find_branch(branch_name, git2::BranchType::Local);
        assert!(branch.is_ok());
    }

    #[test]
    fn test_get_current_branch_name() {
        let (_td, repo) = setup_test_repo();
        let name = get_current_branch_name(&repo).unwrap();
        assert!(name == "main");
    }

    #[test]
    fn test_git_diff_as_string() {
        let (_td, repo) = setup_test_repo();
        let file_path = _td.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Original content").unwrap();
        git_add_all(&repo).unwrap();
        do_commit(&repo, "Initial commit").unwrap();
        let mut file = File::options().append(true).open(&file_path).unwrap();
        writeln!(file, "New line added").unwrap();
        git_add_all(&repo).unwrap();
        let diff_output = git_diff_as_string(&repo).expect("Diff failed");
        assert!(diff_output.contains("+New line added"));
    }

    #[test]
    #[serial]
    fn test_should_push_commit() {
        let (_local_dir, local_repo) = setup_test_repo();
        let (remote_dir, _remote_repo) = setup_test_repo();
        local_repo
            .remote("origin", remote_dir.path().to_str().unwrap())
            .unwrap();
        temp_env::with_vars(
            vec![
                ("QUATI_CUSTOM_LOCAL_GIT_HOST", Some("localhost")),
                ("QUATI_CUSTOM_REMOTE_GIT_HOST", Some("github.com:")),
                ("QUATI_SSH_KEY_PATH", Some("/tmp/fake_key")),
            ],
            || {
                let result = push_branch(&local_repo, "main");
                assert!(result.is_err());
                let err_msg = result.err().unwrap().to_string();
                assert!(
                    err_msg.contains("local push doesn't (yet) support pushing to non-bare repos.")
                        || err_msg.contains("identity")
                );
            },
        )
    }

    #[test]
    #[serial]
    fn test_should_push_commit_without_envs_with_ssh() {
        let (_local_dir, local_repo) = setup_test_repo();
        let (remote_dir, _remote_repo) = setup_test_repo();
        local_repo
            .remote("origin", remote_dir.path().to_str().unwrap())
            .unwrap();
        temp_env::with_vars(
            vec![
                ("QUATI_CUSTOM_LOCAL_GIT_HOST", None::<String>),
                ("QUATI_CUSTOM_REMOTE_GIT_HOST", None::<String>),
                ("QUATI_SSH_KEY_PATH", Some("/tmp/fake_key".to_string())),
            ],
            || {
                let result = push_branch(&local_repo, "main");
                assert!(result.is_err());
                let err_msg = result.err().unwrap().to_string();
                assert!(err_msg.contains("authentication required but no callback set"));
            },
        )
    }

    #[test]
    #[serial]
    fn test_should_push_commit_without_envs() {
        let (_local_dir, local_repo) = setup_test_repo();
        let (remote_dir, _remote_repo) = setup_test_repo();
        local_repo
            .remote("origin", remote_dir.path().to_str().unwrap())
            .unwrap();
        temp_env::with_vars(
            vec![
                ("QUATI_CUSTOM_LOCAL_GIT_HOST", None::<String>),
                ("QUATI_CUSTOM_REMOTE_GIT_HOST", None::<String>),
                ("QUATI_SSH_KEY_PATH", None::<String>),
            ],
            || {
                let result = push_branch(&local_repo, "main");
                assert!(result.is_err());
                let err_msg = result.err().unwrap().to_string();
                assert!(err_msg.contains("authentication required but no callback set"));
            },
        )
    }
}
