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
    Init(Init),
    #[command(about = "List and filter courses")]
    List(List),
    #[command(about = "Aprove a series of courses")]
    Aprove(Aprove),
    #[command(about = "Reject a series of courses")]
    Reject(Reject),
}

#[derive(Args)]
pub(crate) struct Init {
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
pub(crate) struct List {
    // list of possible courses statuses default None
    #[arg(
        short = 's',
        long = "status",
        help = "Status of the courses to list",
        long_help = indoc::indoc!{"
        Status of the courses to list, can be chained, for example:
        course-manager list -s aproved -s rejected
        will list all the courses that are aproved or rejected
        "},
        required = false
    )]
    pub(crate) status: Option<Vec<CourseStatus>>,
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
pub(crate) struct Aprove {
    #[arg(help = "Courses to aprove", required = true)]
    pub(crate) courses: Vec<String>,
    #[arg(
        short = 'r',
        long = "recursive",
        help = "accept courses recursively, so any course that is required by the aproved courses will be aproved too forcibly",
        required = false,
        default_value = "false"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) recursive: bool,
    #[arg(
        short = 'f',
        long = "force",
        help = "force the aproval of the courses, even if they have unaproved requirements",
        required = false,
        default_value = "false"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) force: bool,
}

#[derive(Args)]
pub(crate) struct Reject {
    #[arg(help = "Courses to reject", required = true)]
    pub(crate) courses: Vec<String>,
    #[arg(
        short = 'r',
        long = "recursive",
        help = "reject courses recursively, so any course that requires the rejected courses will be rejected too",
        required = false,
        default_value = "true"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) cascade: bool,
    #[arg(
        short = 'f',
        long = "force",
        help = "force the rejection of the courses, even if they have aproved requirements",
        required = false,
        default_value = "false"
    )]
    #[clap(action = clap::ArgAction::Set)]
    pub(crate) force: bool,
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
    Approved,
    Available,
}

pub(crate) fn to_course_statuses(
    status: &Option<Vec<CourseStatus>>,
) -> Option<Vec<course_manager::courses::CourseStatus>> {
    match status {
        Some(status) => {
            let mut course_statuses: Vec<course_manager::courses::CourseStatus> = Vec::new();
            for s in status {
                match s {
                    CourseStatus::All => return None,
                    CourseStatus::Blocked => {
                        course_statuses.push(course_manager::courses::CourseStatus::Blocked)
                    }
                    CourseStatus::Approved => {
                        course_statuses.push(course_manager::courses::CourseStatus::Approved)
                    }
                    CourseStatus::Available => {
                        course_statuses.push(course_manager::courses::CourseStatus::Available)
                    }
                }
            }
            Some(course_statuses)
        }
        None => None,
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
