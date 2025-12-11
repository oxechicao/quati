// use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
// use predicates::prelude::*;

// #[test]
// fn should_show_help() {
//     let mut cmd = cargo_bin_cmd!("quati");
//     cmd.arg("--help");
//     cmd.assert().success().stdout(predicate::str::contains(
//         "CLI to manage git changes with AI assistance",
//     ));
// }

// #[test]
// fn should_show_start_helper() {
//     let mut cmd = cargo_bin_cmd!("quati");
//     cmd.arg("start").arg("--help");
//     cmd.assert().success().stdout(predicate::str::contains(
//         "Start a new branch locally and remotely",
//     ));
// }
