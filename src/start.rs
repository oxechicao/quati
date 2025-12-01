use crate::git::{
    get_current_branch_name, git_checkout_branch, git_create_branch, git_push_origin,
};

/// Execute the logic for the 'start' action.
pub fn run_start(branch: String, skip_hooks: bool, remove_prefix: bool, no_push: bool) {
    let branch_name = if remove_prefix {
        branch
    } else {
        let prefix = std::env::var("QUATI_PREFIX").unwrap_or_else(|_| "wip/".to_string());
        format!("{}{}", prefix, branch)
    };

    if branch_name == get_current_branch_name() {
        panic!("You are already on branch '{}'", branch_name);
    }

    if !git_checkout_branch(&branch_name) && !git_create_branch(&branch_name) {
        panic!("Failed to create branch '{}'", branch_name);
    }

    if !no_push {
        git_push_origin(skip_hooks);
    }
}
