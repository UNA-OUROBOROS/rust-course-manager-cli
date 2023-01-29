use courses::{Course, CourseStatus};

pub mod courses;
mod error;
mod util;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn courses_files_path() -> Result<std::path::PathBuf, error::Error> {
    Ok(util::get_app_data_dir()?.join("courses"))
}

/// check whethers the app requires initialization, this is usually if the app
/// is being run for the first time, or the course data is missing
pub fn requires_init() -> Result<bool, error::Error> {
    // search for courses.json in the app data dir
    // if it does not exist return true
    // else return false
    return Ok(!courses_files_path()?.join("courses.json").exists());
}

/// initialize the courses list with a list of courses
/// this is usually done when the app is run for the first time
/// or when the course data is missing
/// keep in mind that this will overwrite the existing course data, so an backup should be made if want to keep the old data
pub fn initialize_courses(courses: Vec<courses::Course>) -> Result<(), error::Error> {
    // create the courses.json file in the app data dir
    // write the courses to the file
    let json = serde_json::to_string(&courses).map_err(|e| error::Error::JsonSerialization(e))?;
    let path = courses_files_path()?.join("courses.json");
    match std::fs::write(&path, json) {
        Ok(_) => Ok(()),
        Err(e) => Err(error::Error::CouldNotCreateFile(path, e)),
    }
}

/// gets a filtered list of courses
/// if the filter is None, all courses are returned
/// if the filter is Some, only courses that match the filter are returned
pub fn get_courses(status: Option<CourseStatus>) -> Result<Vec<Course>, error::Error> {
    // TODO
    match status {
        Some(status) => {
            return Ok(Vec::new());
        }
        None => {
            // load courses from courses.json
            let path = courses_files_path()?.join("courses.json");
            let json = std::fs::read_to_string(&path)
                .map_err(|e| error::Error::CouldNotCreateFile(path, e))?;
            let courses =
                serde_json::from_str(&json).map_err(|e| error::Error::JsonDeserialization(e))?;
            return Ok(courses);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
