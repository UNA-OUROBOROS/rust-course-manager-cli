mod cli;
mod util;
use clap::{CommandFactory, Parser};
use course_manager::{approve_courses, courses::to_str, reject_courses, requires_init};

use cli::{to_course_statuses, Cli, Commands, PrintFormat};
use spinoff::{spinners, Spinner};
use tabled::Table;
use util::CourseTable;

use crate::cli::to_table_style;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init(init_courses)) => {
            // check if begins with https
            match init_courses.uri.starts_with("https://") {
                true => {
                    // show a downloading spinner
                    let sp = Spinner::new(spinners::Dots12, "Downloading courses list", None);
                    // download the file
                    match reqwest::blocking::get(&init_courses.uri) {
                        Ok(response) => match response.text() {
                            Ok(text) => {
                                let courses = course_manager::get_courses_from_json(text);
                                match courses {
                                    Ok(courses) => {
                                        match course_manager::initialize_courses(courses) {
                                            Ok(_) => {
                                                sp.success("courses initialized successfully");
                                            }
                                            Err(e) => {
                                                sp.fail(&format!(
                                                    "could not initialize courses: {:#?}",
                                                    e
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        sp.fail(&format!("could not parse courses: {:#?}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                sp.fail(&format!("could not read file: {}", e));
                            }
                        },
                        Err(e) => {
                            sp.fail(&format!("could not download file: {}", e));
                        }
                    }
                }
                false => {
                    let path = init_courses.uri.clone();
                    match std::fs::read_to_string(&path) {
                        Ok(json) => {
                            let courses = course_manager::get_courses_from_json(json);
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
                        Err(e) => {
                            println!("could not open file: {}", e);
                        }
                    }
                }
            }
        }
        Some(Commands::List(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                } else {
                    let status = &list_courses.status;
                    let courses = course_manager::get_courses(to_course_statuses(status));
                    match courses {
                        Ok(courses) => match list_courses.print_format {
                            PrintFormat::Json => {
                                println!("{}", serde_json::to_string_pretty(&courses).unwrap());
                            }
                            PrintFormat::Table => {
                                let courses: Vec<CourseTable> = courses
                                    .iter()
                                    .map(|course| CourseTable {
                                        code: &course.code,
                                        name: &course.name,
                                        status: match course.status {
                                            // fmt::format
                                            Some(status) => to_str(status),
                                            None => "N/A",
                                        },
                                    })
                                    .collect();
                                if courses.is_empty() {
                                    println!("no courses found");
                                } else {
                                    let mut table = Table::new(&courses);
                                    let table =
                                        to_table_style(&mut table, list_courses.table_format);

                                    println!("{}", table);
                                }
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
        Some(Commands::Approve(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                }
                match approve_courses(
                    &list_courses.courses,
                    list_courses.recursive,
                    list_courses.force,
                ) {
                    Ok(_) => {
                        println!("courses approved successfully");
                    }
                    Err(e) => {
                        println!("{:#?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        },
        Some(Commands::Reject(list_courses)) => match requires_init() {
            Ok(requires_init) => {
                if requires_init {
                    println!("please init the courses list first");
                }
                match reject_courses(
                    &list_courses.courses,
                    list_courses.cascade,
                    list_courses.force,
                ) {
                    Ok(_) => {
                        println!("courses rejected successfully");
                    }
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
