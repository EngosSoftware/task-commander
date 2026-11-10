#[cfg(not(target_os = "windows"))]
const NL: &str = "\n";

#[cfg(target_os = "windows")]
const NL: &str = "\r\n";

#[test]
fn _0001() {
    cli_assert::command!()
        .arg("greeting")
        .success()
        .code(0)
        .stdout(format!("Hello world!{NL}"))
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    cli_assert::command!()
        .arg("-p")
        .arg("polish")
        .arg("greeting")
        .success()
        .code(0)
        .stdout(format!("Witaj world!{NL}"))
        .stderr("")
        .execute();
}

#[test]
fn _0003() {
    cli_assert::command!()
        .arg("-p")
        .arg("german")
        .arg("greeting")
        .success()
        .code(0)
        .stdout(format!("Hallo Welt!{NL}"))
        .stderr("")
        .execute();
}
