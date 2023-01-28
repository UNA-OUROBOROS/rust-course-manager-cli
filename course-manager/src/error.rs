use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    UserDirNotFound,
    CouldNotCreatePath(PathBuf, std::io::Error),
    CouldNotCreateFile(PathBuf, std::io::Error),
    CouldNotOpenFile(PathBuf, std::io::Error),
    CouldNotParseConfig(figment::error::Error),
    CouldNotParseJson(serde_json::Error),
}
