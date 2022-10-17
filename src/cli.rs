use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Cli for PMS
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Manage projects
    Project(ProjectArgs),
    /// Manage tasks
    Task(TaskArgs),
    /// Show report
    Report(ReportArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct ProjectArgs {
    #[command(subcommand)]
    pub project_command: ProjectCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectCommand {
    /// List all projects
    List,
    /// Start a new project
    Start {
        /// Project name
        name: String,
    },
    /// Stop existing project
    Stop {
        /// Project id
        id: u32,
    },
    /// Get details of the project
    Get {
        /// Project id
        id: u32,
    },
    /// Select a project so that it can be used later
    Select {
        /// Project id
        id: u32,
    },
}

#[derive(Parser, Debug)]
pub(crate) struct TaskArgs {
    #[command(subcommand)]
    pub task_command: TaskCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum TaskCommand {
    /// List to-do tasks with related project
    List,
    /// Add a new task
    Add {
        /// Task description
        name: String,
        /// When should the task be done
        end_time: Option<String>,
    },
    /// Mark a task as done
    Complete {
        /// Task id
        id: u32,
    },
}

#[derive(Parser, Debug)]
pub(crate) struct ReportArgs {
    #[command(subcommand)]
    pub report_command: ReportCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ReportCommand {
    /// Show tasks for today
    Daily,
    /// Show projects that is about to be rusted
    Rusted,
    /// Show all tasks to be done
    All,
}
