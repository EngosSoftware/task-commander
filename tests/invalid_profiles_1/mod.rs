#[test]
fn _0001() {
    cli_assert::command!()
        .arg("greeting")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: unexpected node: desc\n")
        .execute();
}
