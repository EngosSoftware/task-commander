#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_stdout = "Hello!\n";
    #[cfg(target_os = "windows")]
    let expected_stdout = "Hello!\r\n";
    cli_assert::command!()
        .arg("hello")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
