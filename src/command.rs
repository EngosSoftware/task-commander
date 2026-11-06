use super::errors::{Result, err_command_spawn};
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

/// Executes a given command in shell.
///
/// # Errors
///
/// (tbd)
///
/// # Panics
///
/// TODO Get rid of panics
pub fn execute(arg: impl AsRef<OsStr>, dir: impl AsRef<Path>) -> Result<ExitStatus> {
    let mut command = Command::new(get_shell_command());
    let mut child = command
        .args(get_shell_args())
        .arg(arg)
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| err_command_spawn(e.to_string()))?;
    // It is rare for the following command to fail.
    // If it does, let the code panic.
    Ok(child.wait().expect("failed to wait for child process"))
}

/// Returns a shell command specific for the operating system.
#[cfg(target_os = "macos")]
const fn get_shell_command() -> &'static str {
    "/bin/zsh"
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
fn get_shell_command() -> &'static str {
    "/bin/sh"
}

#[cfg(target_os = "windows")]
#[cfg(target_os = "macos")]
fn get_shell_command() -> &'static str {
    "cmd"
}

/// Returns options required by a shell command.
#[cfg(unix)]
const fn get_shell_args() -> &'static [&'static str] {
    &["-c"]
}

#[cfg(target_os = "windows")]
fn get_shell_args() -> &'static [&'static str] {
    &["/c"]
}
