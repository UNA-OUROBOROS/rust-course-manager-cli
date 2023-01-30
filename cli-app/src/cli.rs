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
    #[command(about = "List and filter courses")]
    ListCourses(ListCourses),
    #[command(about = "Aprove a series of courses")]
    AproveCourses(AproveCourses),
    #[command(about = "Reject a series of courses")]
    RejectCourses(RejectCourses),
}

#[derive(Args)]
pub(crate) struct InitCourses {
    #[arg(required = true, help = "Format of the source")]
    pub(crate) format: Format,
    #[arg(
        required = true,
        help = "URI of the source, can be a local file or a https url"
    )]
    pub(crate) uri: String,
}

#[derive(Args)]
pub(crate) struct ListCourses {
    #[arg(
        short = 's',
        long = "status",
        help = "Status of the courses to list",
        default_value = "all",
        required = false
    )]
    pub(crate) status: CourseStatus,
    // print format, by default table
    #[arg(
        short = 'f',
        long = "format",
        help = "Format of the output",
        default_value = "table",
        required = false
    )]
    pub(crate) print_format: PrintFormat,
}

#[derive(Args)]
pub(crate) struct AproveCourses {
    #[arg(help = "Courses to aprove", required = true)]
    pub(crate) courses: Vec<String>,
}

#[derive(Args)]
pub(crate) struct RejectCourses {
    #[arg(help = "Courses to reject", required = true)]
    pub(crate) courses: Vec<String>,
    #[arg(
        short = 'c',
        long = "cascade",
        help = "reject in cascade the courses that depend on the rejected courses",
        required = false,
        default_value = "false"
    )]
    pub(crate) cascade: bool,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum Format {
    Csv,
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum PrintFormat {
    Markdown,
    Table,
    Json,
    Raw,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum CourseStatus {
    All,
    Blocked,
    Completed,
    Available,
}

pub(crate) fn to_course_status(
    status: CourseStatus,
) -> Option<course_manager::courses::CourseStatus> {
    match status {
        CourseStatus::All => None,
        CourseStatus::Blocked => Some(course_manager::courses::CourseStatus::Blocked),
        CourseStatus::Completed => Some(course_manager::courses::CourseStatus::Completed),
        CourseStatus::Available => Some(course_manager::courses::CourseStatus::Available),
    }
}
