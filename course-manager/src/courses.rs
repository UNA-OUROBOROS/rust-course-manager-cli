use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub code: String,
    pub name: String,
    pub credits: u8,
    pub requirements: Vec<String>,
    pub year: u8,
    pub semester: u8,
    pub is_bachelor: bool,
    // optional status
    pub status: Option<CourseStatus>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CourseStatus {
    Blocked,
    Completed,
    Available,
}

pub fn to_str(status: CourseStatus) -> &'static str {
    match status {
        CourseStatus::Blocked => "Blocked",
        CourseStatus::Completed => "Completed",
        CourseStatus::Available => "Available",
    }
}

// display for course status
impl std::fmt::Display for CourseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_str(*self).fmt(f)
    }
}
