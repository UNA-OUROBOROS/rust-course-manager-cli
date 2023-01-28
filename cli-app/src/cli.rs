use clap::{command, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "Course Manager",
    version = "0.1",
    about = "CLI app for managing courses"
)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Initialize the courses list")]
    InitCourses(InitCourses),
    ListCourses(ListCourses),
}

#[derive(Args)]
pub(crate) struct InitCourses {
    #[arg(required = true, help = "Format of the source")]
    pub(crate) format: Format,
    #[arg(required = true, help = "URL of the source")]
    pub(crate) url: String,
}

#[derive(Args)]
pub(crate) struct ListCourses {
    #[arg(
        short = 's',
        long = "status",
        help = "Status of the courses to list",
        required = false,
    )]
    pub(crate) status: Option<CourseStatus>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum Format {
    Csv,
    Json,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum CourseStatus {
    All,
    Blocked,
    Completed,
    Available,
}
