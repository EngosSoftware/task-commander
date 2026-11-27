#![doc = include_str!("../docs/lib.rs.md")]

pub mod cli;
pub mod command;
pub mod errors;
pub mod loader;
pub mod tasks;

use crate::cli::{CliAction, get_action};
use crate::errors::Result;
use crate::loader::load_definitions;
use crate::tasks::Tasks;
use antex::ColorMode;
use clars::{get_app_and_args, get_more_info_hint};

/// Runs the application.
///
/// # Errors
///
/// Returns failure exit code in case of error.
pub fn run() -> Result<i32> {
    let (app, args) = get_app_and_args();
    match get_action(app, args) {
        CliAction::Run(profile_name, task_name) => {
            let definitions = load_definitions()?;
            let tasks = Tasks::new(&definitions, profile_name)?;
            if let Some(task_name) = task_name {
                let exit_status = tasks.execute(&task_name)?;
                return Ok(exit_status.code().unwrap_or(1));
            }
            tasks.list(ColorMode::default());
        }
        CliAction::ShowVersion(version) => {
            println!("{version}");
        }
        CliAction::ShowHelp(text) => {
            println!("{text}");
        }
        CliAction::Error(text) => {
            eprintln!("{text}");
            eprintln!(
                "{}",
                get_more_info_hint(ColorMode::default(), &["-h", "--help"])
            );
            return Ok(1);
        }
    }
    Ok(0)
}
