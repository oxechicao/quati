use quati::command_runner::{CommandRunner, RealCommandRunner};

#[test]
fn test_should_return_command_output() {
    let mut cmd = RealCommandRunner;
    let result = cmd
        .run("echo", &["Hello, World!"])
        .expect("Failed to run command");

    let res_out = String::from_utf8(result.stdout).ok();
    let res_err = String::from_utf8(result.stderr).ok();
    assert!(result.success);
    assert_eq!(res_out.unwrap().trim(), "Hello, World!");
    assert_eq!(res_err.unwrap(), "");
}
