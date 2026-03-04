use git2::Repository;
use tempfile::tempdir;

use quati::{
    cli::{save, start, update},
    git::create_branch,
};

#[test]
fn test_save() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let path = temp_dir.path();
    Repository::init(path).expect("Failed to initialize git repository");
    let result = save(
        Some("feat".into()),
        Some("auth".into()),
        true,
        false,
        false,
        Some(path),
    );
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

#[test]
fn test_update() {
    let remote_dir = tempdir().expect("Failed to create remote dir");
    Repository::init_bare(remote_dir.path()).expect("Failed to init remote");

    let local_dir = tempdir().expect("Failed to create local dir");
    let path = local_dir.path();

    let repo = Repository::init(local_dir.path()).expect("Failed to init local");
    repo.remote("origin", remote_dir.path().to_str().unwrap())
        .expect("Failed to add remote");

    let signature = repo.signature().unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "initial commit",
        &tree,
        &[],
    )
    .expect("Failed initial commit");

    std::fs::write(path.join("test.txt"), "hello world").unwrap();

    let result = update(
        Some("feat".into()),
        Some("logic".into()),
        true,
        false,
        false,
        Some(path),
    );
    assert!(result.is_ok(), "Update failed: {:?}", result.err());
    let remote_repo = Repository::open_bare(remote_dir.path()).unwrap();
    assert!(remote_repo.find_reference("refs/heads/master").is_ok());
}
