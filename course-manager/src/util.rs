use crate::error;

fn is_debug() -> bool {
    cfg!(debug_assertions)
}

fn app_name() -> String {
    "course-manager".to_string()
}

/// Returns the path of the app user config directory
/// if it does not exist it will be created
///
/// | Error              | Description                           |
/// | ------------------ | ------------------------------------- |
/// | UserDirNotFound    | The user directory could not be found |
/// | CouldNotCreatePath | The app path could not be created     |
fn get_app_config_dir() -> Result<std::path::PathBuf, error::Error> {
    let app_dir = dirs::config_dir()
        .ok_or(error::Error::UserDirNotFound)?
        .join(app_name());
    if !app_dir.exists() {
        let res = std::fs::create_dir(&app_dir);
        if res.is_err() {
            return Err(error::Error::CouldNotCreatePath(
                app_dir,
                res.err().unwrap(),
            ));
        }
    }
    Ok(app_dir)
}

/// Returns the path of the app user data directory
/// this is where application data is stored such as
/// the translation files, resource files, etc.
/// if it does not exist it will be created
/// ## notes
/// on a debug build this will be the current directory
/// on a release build this will be in the respective app data directory
pub(crate) fn get_app_data_dir() -> Result<std::path::PathBuf, error::Error> {
    if is_debug() {
        // get the app dir from manifest dir
        let dir = env!("CARGO_MANIFEST_DIR");
        match std::path::PathBuf::from(dir).parent() {
            Some(path) => {
                return Ok(path.to_path_buf());
            }
            None => Err(error::Error::UserDirNotFound),
        }
    } else {
        let app_dir = dirs::data_local_dir()
            .ok_or(error::Error::UserDirNotFound)?
            .join(app_name());
        if !app_dir.exists() {
            let res = std::fs::create_dir(&app_dir);
            if res.is_err() {
                return Err(error::Error::CouldNotCreatePath(
                    app_dir,
                    res.err().unwrap(),
                ));
            }
        }
        Ok(app_dir)
    }
}
