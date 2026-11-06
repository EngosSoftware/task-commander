//! # Definition of result and errors

use antex::{ColorMode, StyledText, Text};

/// Common result type.
pub type Result<T, E = TaskCommanderError> = std::result::Result<T, E>;

/// Errors.
#[derive(PartialEq, Eq)]
pub enum TaskCommanderError {
    SpawnCommandFailed(String),
    DuplicatedTask(String),
    TaskNotFound(String),
    DependencyCycle(String),
    UnexpectedNode(String),
    ZeroOrOneAttributeAllowed(String),
    MissingDefinitions,
}

impl TaskCommanderError {
    /// Returns a colored error message.
    #[must_use]
    pub fn as_text(&self, cm: ColorMode) -> Text {
        match self {
            Self::SpawnCommandFailed(reason) => Text::new(cm)
                .s("failed to spawn command, reason: ")
                .cyan()
                .s(reason)
                .reset(),
            Self::DuplicatedTask(name) => {
                Text::new(cm).s("duplicated task: ").cyan().s(name).reset()
            }
            Self::TaskNotFound(name) => Text::new(cm).s("task not found: ").cyan().s(name).reset(),
            Self::DependencyCycle(name) => Text::new(cm)
                .s("dependency cycle for task: ")
                .cyan()
                .s(name)
                .reset(),
            Self::UnexpectedNode(name) => {
                Text::new(cm).s("unexpected node: ").cyan().s(name).reset()
            }
            Self::ZeroOrOneAttributeAllowed(name) => Text::new(cm)
                .s("at most one attribute '")
                .cyan()
                .s(name)
                .reset()
                .s("' allowed"),
            Self::MissingDefinitions => {
                Text::new(cm).s("task definitions file is missing or inaccessible")
            }
        }
    }
}

#[must_use]
pub const fn err_command_spawn(reason: String) -> TaskCommanderError {
    TaskCommanderError::SpawnCommandFailed(reason)
}

#[must_use]
pub const fn err_duplicated_task(task_name: String) -> TaskCommanderError {
    TaskCommanderError::DuplicatedTask(task_name)
}

#[must_use]
pub const fn err_task_not_found(name: String) -> TaskCommanderError {
    TaskCommanderError::TaskNotFound(name)
}

#[must_use]
pub const fn err_dependency_cycle(name: String) -> TaskCommanderError {
    TaskCommanderError::DependencyCycle(name)
}

#[must_use]
pub const fn err_unexpected_node(name: String) -> TaskCommanderError {
    TaskCommanderError::UnexpectedNode(name)
}

#[must_use]
pub const fn err_zero_or_one_attribute_allowed(name: String) -> TaskCommanderError {
    TaskCommanderError::ZeroOrOneAttributeAllowed(name)
}

#[must_use]
pub const fn err_definitions_not_found() -> TaskCommanderError {
    TaskCommanderError::MissingDefinitions
}
