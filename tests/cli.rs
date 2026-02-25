use git2::Repository;
use tempfile::tempdir;

use quati::cli::save;

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
