#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_code = 127;
    #[cfg(target_os = "windows")]
    let expected_code = 1;
    cli_assert::command!()
        .arg("run")
        .failure()
        .code(expected_code)
        .stdout("")
        .stderr_fn(|bytes| {
            let text = String::from_utf8(bytes).unwrap();
            text.contains("copacabana")
                && (text.contains("not found") || text.contains("not recognized"))
        })
        .execute();
}
