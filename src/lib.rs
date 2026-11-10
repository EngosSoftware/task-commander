#![doc = include_str!("../docs/lib.rs.md")]

pub mod command;
pub mod errors;
pub mod loader;
pub mod tasks;

use crate::errors::Result;
use crate::loader::load_definitions;
use crate::tasks::Tasks;
use antex::ColorMode;

/// Runs the application.
///
/// # Errors
///
/// Returns [`TaskCommanderError`](errors::TaskCommanderError) when running the application fails.
pub fn run() -> Result<()> {
    let mut profile_name: Option<String> = None;
    let mut task_name: Option<String> = None;
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        if arg == "-p" {
            if let Some(name) = args.next() {
                profile_name = Some(name);
            }
            task_name = args.next();
        } else {
            task_name = Some(arg);
        }
    }
    let definitions = load_definitions()?;
    let tasks = Tasks::new(&definitions, profile_name)?;
    if let Some(task_name) = task_name {
        let exit_status = tasks.execute(&task_name)?;
        std::process::exit(exit_status.code().unwrap_or(1));
    }
    tasks.list(ColorMode::default());
    Ok(())
}
