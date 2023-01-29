mod cli;
mod util;
use clap::{CommandFactory, Parser};
use course_manager::{approve_courses, requires_init};

use cli::{to_course_status, Cli, Commands, PrintFormat};

use crate::cli::Format;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::InitCourses(init_courses)) => {
            // check if begins with https
            match init_courses.uri.starts_with("https://") {
                true => {
                    println!("HTTPS is not supported yet");
                }
                false => {
                    let courses = match init_courses.format {
                        Format::Csv => {
                            print!("CSV is not supported yet");
                            todo!()
                        }
                        Format::Json => {
                            let path = init_courses.uri.clone();
                            course_manager::get_courses_from_json(path)
                        }
                    };
                    match courses {
                        Ok(courses) => match course_manager::initialize_courses(courses) {
                            Ok(_) => {
                                println!("courses initialized successfully");
                            }
                            Err(e) => {
                                println!("{:#?}", e);
                            }
                        },
                        Err(e) => {
                            println!("{:#?}", e);
                        }
                    }
                }
            }
        }
        Some(Commands::ListCourses(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                } else {
                    let status = list_courses.status;
                    let courses = course_manager::get_courses(to_course_status(status));
                    match courses {
                        Ok(courses) => match list_courses.print_format {
                            PrintFormat::Json => {
                                println!("{}", serde_json::to_string_pretty(&courses).unwrap());
                            }
                            PrintFormat::Table => {
                                let mut table = prettytable::Table::new();
                                table.add_row(prettytable::row!["Code", "Name", "Status"]);
                                for course in courses {
                                    table.add_row(prettytable::row![
                                        course.code,
                                        course.name,
                                        match course.status {
                                            Some(status) => status.to_string(),
                                            None => "N/A".to_string(),
                                        }
                                    ]);
                                }
                                table.printstd();
                            }
                            PrintFormat::Raw => {
                                println!("{:#?}", courses);
                            }
                        },
                        Err(e) => {
                            println!("{:#?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        },
        Some(Commands::AproveCourses(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                }
                match approve_courses(&list_courses.courses) {
                    Ok(_) => {
                        println!("courses approved successfully");
                    },
                    Err(e) => {
                        println!("{:#?}", e);
                    }
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
