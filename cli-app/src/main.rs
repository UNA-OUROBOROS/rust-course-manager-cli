mod cli;
mod util;
use clap::{CommandFactory, Parser};
use course_manager::requires_init;

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
        Some(Commands::ListCourses(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                } else {
                    let status = list_courses.status;
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
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        },
        None => {
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
        }
    }
}
