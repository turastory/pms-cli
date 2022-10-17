use crate::cli::*;
use anyhow::{Context, Result};
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};

pub mod cli;

#[derive(Debug)]
struct Project {
    name: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut data: Data = Data::from_data_file("data");

    match args.command {
        Command::Project(command) => data.handle_project(command.project_command),
        Command::Task(command) => data.handle_task(command.task_command),
        Command::Report(command) => data.handle_report(command.report_command),
    }

    Ok(())
}

struct Data {
    filename: String,
    projects: Vec<Project>,
}

impl Data {
    fn new() -> Self {
        Data {
            filename: "data".to_string(),
            projects: vec![],
        }
    }

    fn from_data_file(filename: &str) -> Self {
        std::fs::File::open(filename)
            .map(|file| BufReader::new(file))
            .map(|reader| Data {
                filename: filename.to_string(),
                projects: reader
                    .lines()
                    .map(|line| line.unwrap())
                    .map(|line| Project { name: line })
                    .collect(),
            })
            .with_context(|| format!("Failed to read file {}", filename))
            .unwrap_or_default()
    }

    fn save_data_file(&self) -> Result<()> {
        OpenOptions::new()
            .write(true)
            .open(&self.filename)
            .map(|file| BufWriter::new(file))
            .map(|mut writer| {
                self.projects.iter().for_each(|project| {
                    writeln!(writer, "{}", project.name).unwrap();
                })
            })
            .with_context(|| format!("Failed to write to file {}", self.filename))
    }

    fn handle_project(&mut self, command: ProjectCommand) {
        match command {
            ProjectCommand::List => {
                println!("List of projects:");
                self.projects
                    .iter()
                    .map(|project| format!(" - {}", project.name))
                    .for_each(|name| {
                        println!("{}", name);
                    });
            }
            ProjectCommand::Start { name } => {
                let name = name.trim().to_string();
                if self.projects.iter().any(|project| project.name == name) {
                    println!("Project {} already exists", name);
                } else {
                    println!("Starting a new project: {}", name);
                    self.projects.push(Project { name });
                    self.save_data_file().unwrap();
                }
            }
            ProjectCommand::Stop { id } => println!("Stopping project with id {}", id),
            ProjectCommand::Get { id } => println!("Getting details of project with id {}", id),
            ProjectCommand::Select { id } => println!("Selecting project with id {}", id),
        }
    }

    fn handle_task(&mut self, command: TaskCommand) {
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

    fn handle_report(&mut self, command: ReportCommand) {
        match command {
            ReportCommand::Daily => println!("Showing tasks for today"),
            ReportCommand::Rusted => println!("Showing projects that is about to be rusted"),
            ReportCommand::All => println!("Showing all tasks to be done"),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
