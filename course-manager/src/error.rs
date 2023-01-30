use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    UserDirNotFound,
    CouldNotCreatePath(PathBuf, std::io::Error),
    CouldNotCreateFile(PathBuf, std::io::Error),
    CouldNotOpenFile(PathBuf, std::io::Error),
    CouldNotParseConfig(figment::error::Error),
    JsonSerialization(serde_json::Error),
    JsonDeserialization(serde_json::Error),
    CourseAlreadyApproved(String),
    CourseDoesNotExist(String),
    CourseNotApproved(String),
}
