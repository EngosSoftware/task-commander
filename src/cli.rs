//! Command-line utilities

use antex::{StyledText, Text};
use clars::{Clar, ClarArgument, ClarOption};

const NAME: &str = env!("CARGO_PKG_NAME");

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ProfileName = Option<String>;

pub type TaskName = Option<String>;

/// Command-line action.
pub enum CliAction {
    Run(
        /// Profile name.
        ProfileName,
        /// Task name.
        TaskName,
    ),
    ShowHelp(
        /// Help text.
        Text,
    ),
    ShowVersion(
        /// Version description.
        Text,
    ),
    Error(
        /// Error details.
        Text,
    ),
}

fn get_cli(app: String) -> Clar {
    Clar::new(app).description(DESCRIPTION).options_arguments(
        [
            ClarOption::short("profile", 'p')
                .long_label("profile")
                .takes_value("PROFILE")
                .help("Profile name"),
            ClarOption::short("version", 'V')
                .long_label("version")
                .standalone()
                .help("Print version"),
            ClarOption::short("help", 'h')
                .long_label("help")
                .standalone()
                .help("Print help"),
        ],
        [ClarArgument::new("task").help("Task name")],
    )
}

#[must_use]
pub fn get_action(app: String, args: Vec<String>) -> CliAction {
    match get_cli(app).resolve(args) {
        Ok(matches) => {
            if matches.is_short("help") {
                return CliAction::ShowHelp(matches.get_help());
            }
            if matches.is_long("help") {
                return CliAction::ShowHelp(matches.get_help_long());
            }
            if matches.is_present("version") {
                let version = Text::default()
                    .bright_cyan()
                    .bold()
                    .s(NAME)
                    .r(" v")
                    .s(VERSION);
                return CliAction::ShowVersion(version);
            }
            let profile_name: ProfileName = matches.get_first_value("profile");
            let task_name: TaskName = matches.get_first_value("task");
            CliAction::Run(profile_name, task_name)
        }
        Err(diagnostic) => CliAction::Error(diagnostic.text().clone()),
    }
}
