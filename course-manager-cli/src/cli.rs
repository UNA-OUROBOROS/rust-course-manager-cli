use clap::{command, Args, Parser, Subcommand, ValueEnum};
use tabled::{Style, Table};

#[derive(Parser)]
#[command(
    name = "Course Manager",
    author = "Ouroboros",
    version,
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
    #[arg(
        required = true,
        help = "URI of the source, can be a local file or a https url, for more info use --help",
        long_help = indoc::indoc!{"
        URI of the source, can be a local file or a https url.
        for example you use the following url to get the courses from the course manager repo:
        https://raw.githubusercontent.com/UNA-OUROBOROS/course-manager-data/master/ING-SIST-UNA-V1.json
        like this:
        course-manager init-courses https://raw.githubusercontent.com/UNA-OUROBOROS/course-manager-data/master/ING-SIST-UNA-V1.json
        or you can use a local file like this:
        course-manager init-courses ./ING-SIST-UNA-V1.json
        "}
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
    // table format, used only if the format is table
    #[arg(
        short = 't',
        long = "table-format",
        help = "Format of the table, only used if the format is table",
        default_value = "pretty",
        required = false,
        default_value = "rounded"
    )]
    pub(crate) table_format: TableStyle,
}

#[derive(Args)]
pub(crate) struct AproveCourses {
    #[arg(help = "Courses to aprove", required = true)]
    pub(crate) courses: Vec<String>,
    #[arg(
        short = 'c',
        long = "cascade",
        help = "accept courses in cascade, so any course that is required gets accepted",
        required = false,
        default_value = "true"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) cascade: bool,
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
        default_value = "true"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) cascade: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum PrintFormat {
    Table,
    Json,
    Raw,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum TableStyle {
    Ascii,
    Modern,
    Sharp,
    Rounded,
    Extended,
    Psql,
    Markdown,
    Rst,
    Dots,
    AsciiRounded,
    Blank,
    Empty,
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

pub(crate) fn to_table_style(table: &mut Table, style: TableStyle) -> &mut Table {
    match style {
        TableStyle::Ascii => table.with(Style::ascii()),
        TableStyle::Modern => table.with(Style::modern()),
        TableStyle::Sharp => table.with(Style::sharp()),
        TableStyle::Rounded => table.with(Style::rounded()),
        TableStyle::Extended => table.with(Style::extended()),
        TableStyle::Psql => table.with(Style::psql()),
        TableStyle::Markdown => table.with(Style::markdown()),
        TableStyle::Rst => table.with(Style::re_structured_text()),
        TableStyle::Dots => table.with(Style::dots()),
        TableStyle::AsciiRounded => table.with(Style::ascii_rounded()),
        TableStyle::Blank => table.with(Style::blank()),
        TableStyle::Empty => table.with(Style::empty()),
    }
}