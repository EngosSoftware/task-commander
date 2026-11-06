#[test]
fn _0001() {
    cli_assert::command!()
        .arg("hello")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: at most one attribute 'dir' allowed\n")
        .execute();
}
