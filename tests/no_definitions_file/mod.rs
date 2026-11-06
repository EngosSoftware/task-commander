#[test]

fn _0001() {
    cli_assert::command!()
        .arg("run")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: task definitions file is missing or inaccessible\n")
        .execute();
}
