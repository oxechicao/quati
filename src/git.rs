use std::process::Command;

/// Get the current branch name to be used as default branch name when run quati start.
/// If the git command fails, panic with an error message.
pub fn get_current_branch_name() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        branch_name
    } else {
        eprintln!(
            "Error executing git command:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        panic!("Failed to get current branch name.\n Are you in a git repository?");
    }
}

/// Push the current branch to origin, with an option to skip git hooks.
pub fn git_push_origin(skip_hooks: bool) {
    let push = Command::new("git")
        .args([
            "push",
            "-u",
            "origin",
            if skip_hooks { "--no-verify" } else { "" },
        ])
        .output()
        .expect("Failed to push branch to origin");

    if push.status.success() {
        return println!("Pushed branch to origin");
    }

    eprintln!(
        "Error pushing branch to origin:\n{}",
        String::from_utf8_lossy(&push.stderr)
    );
}

pub fn git_checkout_branch(branch_name: &str) -> bool {
    let checkout_branch = Command::new("git")
        .args(["checkout", branch_name])
        .output()
        .expect("Failed to checkout branch");

    return checkout_branch.status.success();
}

pub fn git_create_branch(branch_name: &str) -> bool {
    let create_branch = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .expect("Failed to create branch");

    return create_branch.status.success();
}

mod tests {
    use super::*;

    #[test]
    fn test_should_return_empty_name_when_not_in_git_repo() {
        let branch_name = get_current_branch_name();
        assert!(branch_name.is_empty());
    }

    #[test]
    fn test_should_return_the_current_branch_name() {
        let branch_name = get_current_branch_name();
        assert!(!branch_name.is_empty());
    }
}
