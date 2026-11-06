#![cfg(not(target_os = "windows"))]

#[test]
fn _0001() {
    // Given: Command definition that displays the directory name.
    //  When: Provided custom working directory to the command.
    //  Then: The command should be executed in the custom working directory.
    //        For this test it should be 'task-commander' which is the project directory.
    cli_assert::command!()
        .arg("run")
        .success()
        .code(0)
        .stdout("task-commander\n")
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    // Given: Command definition that displays the directory name.
    //  When: Provided non-existing custom working directory to the command.
    //  Then: The command should fail with an error.
    cli_assert::command!()
        .arg("invalid-dir")
        .failure()
        .code(1)
        .stdout("")
        .stderr("error: failed to spawn command, reason: No such file or directory (os error 2)\n")
        .execute();
}
