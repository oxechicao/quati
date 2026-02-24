// use std::{env, io::Result, process::Command};
//
// use git2::Repository;
//
// #[test]
// fn test_start() {
//     let temp_dir = tempfile::tempdir().unwrap();
//     let repo = Repository::init(temp_dir.path()).unwrap();
//     let a = Command::new("pwd")
//         .output()
//         .expect("Failed to initialize git repository");
//     let path = env::current_dir()?;
//     println!("The current directory is {}", path.display());
//     let stdout = str::from_utf8(&a.stdout).unwrap();
//     print!("The current directory is: {}", stdout);
// }
