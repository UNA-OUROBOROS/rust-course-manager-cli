use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub credits: u8,
    pub requirements: Vec<String>,
    pub semester: u8,
    pub cicle: u8,
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
