use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

// display for course status
impl std::fmt::Display for CourseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseStatus::Blocked => write!(f, "Blocked"),
            CourseStatus::Completed => write!(f, "Completed"),
            CourseStatus::Available => write!(f, "Available"),
        }
    }
}
