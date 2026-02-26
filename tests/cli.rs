use git2::Repository;
use tempfile::tempdir;

use quati::{
    cli::{save, start},
    git::create_branch,
};

#[test]
fn test_save() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let path = temp_dir.path();
    Repository::init(path).expect("Failed to initialize git repository");
    let result = save(Some("feat".into()), Some("auth".into()), Some(path));
    assert!(
        result.is_ok(),
        "Expected save to succeed, got error: {:?}",
        result.err()
    );
}

#[test]
fn test_start() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let path = temp_dir.path();
    let repo = Repository::init(path).expect("Failed to initialize git repository");
    let signature = repo.signature().unwrap();
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[], // Sem pais (parent commits)
    )
    .expect("Failed to create initial commit");
    create_branch(&repo, "main").expect("Failed to create main branch");
    let result = start(
        Some("test_branch".to_string()),
        None,
        true,
        true,
        Some(path),
    );

    assert!(
        result.is_ok(),
        "Expected start to succeed, got error: {:?}",
        result.err()
    );
}
