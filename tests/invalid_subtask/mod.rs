#[test]
fn _0001() {
    cli_assert::command!()
        .arg("parent")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: task not found: child\n")
        .execute();
}
