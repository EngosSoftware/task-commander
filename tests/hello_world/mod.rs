#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_stdout = "Hello world!\n";
    #[cfg(target_os = "windows")]
    let expected_stdout = "Hello world!\r\n";
    cli_assert::command!()
        .arg("greeting")
        .success()
        .code(0)
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
