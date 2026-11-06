#[test]

fn _0001() {
    cli_assert::command!()
        .arg("run")
        .success()
        .code(0)
        .stdout("")
        .stderr("")
        .execute();
}
