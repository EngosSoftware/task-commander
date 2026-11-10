#[test]
fn _0001() {
    #[cfg(target_os = "windows")]
    cli_assert::command!()
        .arg("run")
        .failure()
        .code(1)
        .stdout("")
        .stderr_fn(|bytes| {
            let text = String::from_utf8(bytes).unwrap();
            text.contains("copacabana") && text.contains("not recognized")
        })
        .execute();
}

#[test]
fn _0002() {
    #[cfg(target_os = "linux")]
    cli_assert::command!()
        .arg("LANG=C run")
        .failure()
        .code(127)
        .stdout("")
        .stderr_fn(|bytes| {
            let text = String::from_utf8(bytes).unwrap();
            text.contains("copacabana") && text.contains("not found")
        })
        .execute();
}

#[test]
fn _0003() {
    #[cfg(target_os = "macos")]
    cli_assert::command!()
        .arg("run")
        .failure()
        .code(127)
        .stdout("")
        .stderr_fn(|bytes| {
            let text = String::from_utf8(bytes).unwrap();
            text.contains("copacabana") && text.contains("not found")
        })
        .execute();
}
