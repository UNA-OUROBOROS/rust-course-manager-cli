mod cli;
mod util;
use clap::{CommandFactory, Parser};

use cli::{Cli, Commands};

use crate::cli::Format;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::InitCourses(init_courses)) => {
            println!(
                "format: {}, url: {}",
                match init_courses.format {
                    Format::Csv => "csv",
                    Format::Json => "json",
                },
                init_courses.url
            );
        }
        Some(Commands::ListCourses(list_courses)) => {
            let status = list_courses.status.unwrap_or(cli::CourseStatus::All);
            println!(
                "status: {:?}",
                match status {
                    cli::CourseStatus::All => "all",
                    cli::CourseStatus::Blocked => "blocked",
                    cli::CourseStatus::Completed => "completed",
                    cli::CourseStatus::Available => "available",
                }
            );
        }
        None => {
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
        }
    }
}
