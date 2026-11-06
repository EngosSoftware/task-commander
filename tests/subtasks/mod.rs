#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_stdout = "Hello parent!\n";
    #[cfg(target_os = "windows")]
    let expected_stdout = "\\\"Hello parent!\\\"\r\n";
    cli_assert::command!()
        .arg("parent")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
