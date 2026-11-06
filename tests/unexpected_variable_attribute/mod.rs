#[test]
fn _0001() {
    cli_assert::command!()
        .arg("hello")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: unexpected node: name\n")
        .execute();
}
