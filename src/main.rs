use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Cli for PMS
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Manage projects
    Project(ProjectArgs),
    /// Manage tasks
    Task(TaskArgs),
    /// Show report
    Report(ReportArgs),
}

#[derive(Parser, Debug)]
struct ProjectArgs {
    #[command(subcommand)]
    project_command: ProjectCommand,
}

#[derive(Subcommand, Debug)]
enum ProjectCommand {
    /// List all projects
    List,
    /// Start a new project
    Start,
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
struct TaskArgs {
    #[command(subcommand)]
    task_command: TaskCommand,
}

#[derive(Subcommand, Debug)]
enum TaskCommand {
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
struct ReportArgs {
    #[command(subcommand)]
    report_command: ReportCommand,
}

#[derive(Subcommand, Debug)]
enum ReportCommand {
    /// Show tasks for today
    Daily,
    /// Show projects that is about to be rusted
    Rusted,
    /// Show all tasks to be done
    All,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Project(command) => handle_project(command.project_command),
        Command::Task(command) => handle_task(command.task_command),
        Command::Report(command) => handle_report(command.report_command),
    }
}

fn handle_project(command: ProjectCommand) {
    match command {
        ProjectCommand::List => println!("Listing projects"),
        ProjectCommand::Start => println!("Starting a new project"),
        ProjectCommand::Stop { id } => println!("Stopping project with id {}", id),
        ProjectCommand::Get { id } => println!("Getting details of project with id {}", id),
        ProjectCommand::Select { id } => println!("Selecting project with id {}", id),
    }
}

fn handle_task(command: TaskCommand) {
    match command {
        TaskCommand::List => println!("Listing tasks"),
        TaskCommand::Add { name, end_time } => {
            println!(
                "Adding a new task with name {} and end time {}",
                name,
                end_time.unwrap_or("None".to_string())
            )
        }
        TaskCommand::Complete { id } => println!("Completing task with id {}", id),
    }
}

fn handle_report(command: ReportCommand) {
    match command {
        ReportCommand::Daily => println!("Showing tasks for today"),
        ReportCommand::Rusted => println!("Showing projects that is about to be rusted"),
        ReportCommand::All => println!("Showing all tasks to be done"),
    }
}
