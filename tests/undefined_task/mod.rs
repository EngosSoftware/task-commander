#[test]
fn _0001() {
    cli_assert::command!()
        .arg("build")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: task not found: build\n")
        .execute();
}
