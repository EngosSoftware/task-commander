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
    let definitions = load_definitions()?;
    let tasks = Tasks::new(&definitions)?;
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 1 {
        let task_name = &args[0];
        let exit_status = tasks.execute(task_name)?;
        std::process::exit(exit_status.code().unwrap_or(1));
    }
    tasks.list(ColorMode::default());
    Ok(())
}
