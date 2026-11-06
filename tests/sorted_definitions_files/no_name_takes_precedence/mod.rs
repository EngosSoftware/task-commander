#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_stdout = "My name is 'a'!\n";
    #[cfg(target_os = "windows")]
    let expected_stdout = "\\\"My name is 'a'!\\\"\r\n";
    cli_assert::command!()
        .arg("run")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
