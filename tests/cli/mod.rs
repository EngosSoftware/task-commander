#[cfg(target_os = "windows")]
fn win(input: &str) -> String {
    input.replace("task-commander ", "task-commander.exe ")
}

#[test]
fn _0001() {
    let expected_stderr = r"error: unexpected option '-k' found

Usage: task-commander [OPTIONS]  [ARGS]

For more information, try '-h' or '--help'
";
    #[cfg(target_os = "windows")]
    let expected_stderr = win(expected_stderr);
    cli_assert::command!()
        .arg("-k")
        .failure()
        .code(1)
        .stdout("")
        .stderr(expected_stderr)
        .execute();
}

#[test]
fn _0002() {
    let expected_stdout = "task-commander v0.1.0\n";
    cli_assert::command!()
        .arg("-V")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}

#[test]
fn _0003() {
    let expected_stdout = r"TaskCommander

Usage: task-commander [OPTIONS]  [ARGS]

Arguments:
  [TASK]  Task name

Options:
  -p, --profile <PROFILE>  Profile name
  -V, --version            Print version
  -h, --help               Print help

";
    #[cfg(target_os = "windows")]
    let expected_stdout = win(expected_stdout);
    cli_assert::command!()
        .arg("-h")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}

#[test]
fn _0004() {
    let expected_stdout = r"TaskCommander

Usage: task-commander [OPTIONS]  [ARGS]

Arguments:
  [TASK]  Task name

Options:
  -p, --profile <PROFILE>  Profile name
  -V, --version            Print version
  -h, --help               Print help

";
    #[cfg(target_os = "windows")]
    let expected_stdout = win(expected_stdout);
    cli_assert::command!()
        .arg("--help")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
