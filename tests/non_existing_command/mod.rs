#[test]
fn _0001() {
    #[cfg(target_os = "macos")]
    let (expected_stderr, expected_code) = ("zsh:1: command not found: copacabana\n", 127);
    #[cfg(target_os = "linux")]
    let (expected_stderr, expected_code) = ("/bin/sh: 1: copacabana: not found\n", 127);
    #[cfg(target_os = "windows")]
    let (expected_stderr, expected_code) = (
        "'copacabana' is not recognized as an internal or external command,\r\noperable program or batch file.\r\n",
        1,
    );
    cli_assert::command!()
        .arg("run")
        .failure()
        .code(expected_code)
        .stdout("")
        .stderr(expected_stderr)
        .execute();
}
