use super::errors::{
    Result, err_dependency_cycle, err_duplicated_task, err_task_not_found, err_unexpected_node,
    err_zero_or_one_attribute_allowed,
};
use crate::command::execute;
use antex::{ColorMode, StyledText, Text};
use petgraph::Graph;
use petgraph::algo::toposort;
use petgraph::graph::NodeIndex;
use std::collections::{BTreeMap, HashMap};
use std::process::ExitStatus;

/// An action to be performed.
#[derive(Debug, Clone)]
pub enum Action {
    Command(
        /// Shell command to be executed.
        String,
        /// Custom working directory.
        String,
    ),
    Task(
        /// Subtask name.
        String,
    ),
}

/// A task.
#[derive(Debug, Default, Clone)]
pub struct Task {
    /// Task name.
    name: String,
    /// Task description.
    description: Option<String>,
    /// Expected actions to be performed to complete this task.
    actions: Vec<Action>,
    /// Shell commands to be executed to complete this task.
    commands: Vec<(String, String)>,
}

impl Task {
    /// Creates a new task with a give name.
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Returns task name.
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns task description.
    #[must_use]
    pub const fn get_description(&self) -> &Option<String> {
        &self.description
    }

    /// Sets task description.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Adds a new command to this task.
    pub fn add_command(&mut self, command: String, dir: String) {
        self.actions.push(Action::Command(command, dir));
    }

    /// Adds a new subtask to this task.
    pub fn add_task(&mut self, task: String) {
        self.actions.push(Action::Task(task));
    }
}

/// A collection of tasks.
#[derive(Debug)]
pub struct Tasks {
    tasks: BTreeMap<String, Task>,
    variables: HashMap<String, String>,
}

impl Tasks {
    /// Creates executable tasks from definitions.
    ///
    /// # Errors
    ///
    /// (tbd)
    ///
    /// # Panics
    ///
    /// TODO: Get rid of panics
    pub fn new(definitions: &str) -> Result<Self> {
        let mut tasks = Self::parse_definitions(definitions)?;
        let ordered_names = Self::sort_tasks_topologically(&tasks)?;
        for name in &ordered_names {
            let mut commands = vec![];
            let task = tasks.tasks.get(name).unwrap(); // TODO get rid of unwrap
            for action in &task.actions {
                match action {
                    Action::Command(command, dir) => {
                        commands.push((interpolate(command, &tasks.variables), dir.clone()));
                    }
                    Action::Task(name) => {
                        let subtask = tasks.tasks.get(name).unwrap(); // TODO get rid of unwrap
                        commands.append(&mut subtask.commands.clone());
                    }
                }
            }
            let task = tasks.tasks.get_mut(name).unwrap(); // TODO get rid of unwrap
            task.commands.clear();
            task.commands.append(&mut commands);
        }
        Ok(tasks)
    }

    /// Executes all commands of the given task.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no task with specified name.
    pub fn execute(&self, name: &str) -> Result<ExitStatus> {
        let mut exit_status = ExitStatus::default();
        if let Some(task) = self.tasks.get(name) {
            for (command, dir) in &task.commands {
                exit_status = execute(command, dir)?;
                if !exit_status.success() {
                    break;
                }
            }
        } else {
            return Err(err_task_not_found(name.to_string()));
        }
        Ok(exit_status)
    }

    /// Displays a list of tasks.
    pub fn list(&self, cm: ColorMode) {
        let width = self.tasks.keys().map(String::len).max().unwrap_or_default();
        println!(
            "{}",
            Text::new(cm)
                .bold()
                .underline()
                .s("Available tasks")
                .reset()
        );
        for task in self.tasks.values() {
            let name = task.get_name();
            if let Some(description) = task.get_description() {
                print!(
                    "  {}  {}",
                    Text::new(cm).bright_green().align_left(name, width).reset(),
                    description.lines().take(1).collect::<String>()
                );
            } else {
                print!("  {}", Text::new(cm).bright_green().s(name).reset());
            }
            println!();
        }
    }

    /// Parses task definitions.
    fn parse_definitions(definitions: &str) -> Result<Self> {
        let mut tasks = Self {
            tasks: BTreeMap::new(),
            variables: HashMap::new(),
        };
        let root_node = diml::parse(definitions).expect("failed to parse the task file");
        for top_level_node in root_node.children() {
            match top_level_node.name() {
                "note" => {} // Global notes are treated as a comment.
                "vars" => {
                    // Global variables.
                    for variable_node in top_level_node.children() {
                        for task_attribute_node in variable_node.children() {
                            match task_attribute_node.name() {
                                "note" => {} // Variable notes are treated as a comment.
                                other => return Err(err_unexpected_node(other.to_string())),
                            }
                        }
                        tasks.variables.insert(
                            variable_node.name().to_string(),
                            variable_node.content().trim().to_string(),
                        );
                    }
                }
                "tasks" => {
                    // Task definitions.
                    for task_node in top_level_node.children() {
                        let task_name = task_node.name().to_string();
                        if tasks.tasks.contains_key(&task_name) {
                            return Err(err_duplicated_task(task_name));
                        }
                        let mut task = Task::new(task_name.clone());
                        for task_attribute_node in task_node.children() {
                            match task_attribute_node.name() {
                                "note" => {} // Task notes are treated as a comment.
                                "desc" => {
                                    task.set_description(
                                        task_attribute_node.content().trim().to_string(),
                                    );
                                }
                                "task" => {
                                    task.add_task(task_attribute_node.content().trim().to_string());
                                }
                                "cmd" => {
                                    let command_text = task_attribute_node.content().trim();
                                    let mut dir = None;
                                    for command_attribute_node in task_attribute_node.children() {
                                        match command_attribute_node.name() {
                                            "note" => {} // Command notes are treated as a comment.
                                            v @ "dir" if dir.is_some() => {
                                                return Err(err_zero_or_one_attribute_allowed(
                                                    v.to_string(),
                                                ));
                                            }
                                            "dir" => {
                                                dir = Some(
                                                    command_attribute_node
                                                        .content()
                                                        .trim()
                                                        .to_string(),
                                                );
                                            }
                                            other => {
                                                return Err(err_unexpected_node(other.to_string()));
                                            }
                                        }
                                    }
                                    task.add_command(
                                        command_text.to_string(),
                                        dir.unwrap_or_else(|| ".".to_string()),
                                    );
                                }
                                other => return Err(err_unexpected_node(other.to_string())),
                            }
                        }
                        tasks.tasks.insert(task_name.clone(), task);
                    }
                }
                other => return Err(err_unexpected_node(other.to_string())),
            }
        }
        Ok(tasks)
    }

    /// Sorts tasks topologically, detect non-existing subtasks and dependency cycles.
    fn sort_tasks_topologically(tasks: &Self) -> Result<Vec<String>> {
        let mut graph = Graph::<String, ()>::new();
        let mut name_by_index: HashMap<NodeIndex, String> = HashMap::new();
        let mut index_by_name: HashMap<String, NodeIndex> = HashMap::new();
        // Add all nodes to the graph.
        for (name, _) in tasks {
            let node_index = graph.add_node(name.to_owned());
            name_by_index.insert(node_index, name.to_owned());
            index_by_name.insert(name.to_owned(), node_index);
        }
        // Add directed edges to the graph, from child node to parent node.
        for (name, task) in tasks {
            let parent_index = index_by_name.get(name).copied().unwrap();
            for action in &task.actions {
                if let Action::Task(name) = action {
                    let Some(child_index) = index_by_name.get(name).copied() else {
                        return Err(err_task_not_found(name.to_owned()));
                    };
                    graph.add_edge(child_index, parent_index, ());
                }
            }
        }
        // Sort topologically, detect cycles.
        match toposort(&graph, None) {
            Ok(indexes) => Ok(indexes
                .iter()
                .map(|index| name_by_index.get(index).cloned().unwrap())
                .collect()),
            Err(cycle) => Err(err_dependency_cycle(
                name_by_index.get(&cycle.node_id()).cloned().unwrap(),
            )),
        }
    }

    fn iter(&self) -> std::collections::btree_map::Iter<'_, String, Task> {
        self.tasks.iter()
    }
}

/// Interpolates variables in the given string.
fn interpolate(input: &str, global_variables: &HashMap<String, String>) -> String {
    let mut interpolated = input.to_string();
    for (name, value) in global_variables {
        let pattern = format!("{{{{{name}}}}}");
        interpolated = interpolated.replace(&pattern, value);
    }
    interpolated
}

impl<'a> IntoIterator for &'a Tasks {
    type Item = (&'a String, &'a Task);
    type IntoIter = std::collections::btree_map::Iter<'a, String, Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
