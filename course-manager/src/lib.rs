use courses::{Course, CourseStatus};

pub mod courses;
mod error;
#[cfg(test)]
mod tests;
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
    let path = courses_files_path()?;
    if !&path.exists() {
        std::fs::create_dir(&path)
            .map_err(|e| error::Error::CouldNotCreatePath(path.clone(), e))?;
    }
    let path = path.join("courses.json");
    let json = serde_json::to_string(&courses).map_err(|e| error::Error::JsonSerialization(e))?;
    std::fs::write(&path, json).map_err(|e| error::Error::CouldNotCreateFile(path, e))?;
    // additionally create a aproved.json whcih is a vector of strings
    let path = courses_files_path()?.join("approved.json");
    let json = serde_json::to_string(&Vec::<String>::new())
        .map_err(|e| error::Error::JsonSerialization(e))?;
    std::fs::write(&path, json).map_err(|e| error::Error::CouldNotCreateFile(path, e))?;
    Ok(())
}

/// gets a filtered list of courses
/// if the filter is None, all courses are returned
/// if the filter is Some, only courses that match the filter are returned
pub fn get_courses(status: Option<CourseStatus>) -> Result<Vec<Course>, error::Error> {
    // load courses from courses.json
    let path = courses_files_path()?.join("courses.json");
    let json =
        std::fs::read_to_string(&path).map_err(|e| error::Error::CouldNotOpenFile(path, e))?;
    let courses: Vec<Course> =
        serde_json::from_str(&json).map_err(|e| error::Error::JsonDeserialization(e))?;
    match status {
        Some(status) => {
            // load approved.json
            let path = courses_files_path()?.join("approved.json");
            let json = std::fs::read_to_string(&path)
                .map_err(|e| error::Error::CouldNotOpenFile(path, e))?;
            let approved: Vec<String> =
                serde_json::from_str(&json).map_err(|e| error::Error::JsonDeserialization(e))?;
            let mut filtered_courses: Vec<Course> = Vec::new();
            match status {
                CourseStatus::Completed => {
                    // fetch all courses that are in approved.json
                    for course in courses {
                        if approved.contains(&course.code) {
                            filtered_courses.push(course);
                        }
                    }
                }
                CourseStatus::Blocked | CourseStatus::Available => {
                    let requires_aproved = status == CourseStatus::Available;
                    // fetch all courses that are not in approved.json
                    for course in courses {
                        if !approved.contains(&course.code) {
                            // and if that the status is the same as the filter
                            let requires_met = requirements_met(&course, &approved);
                            if requires_aproved == requires_met {
                                filtered_courses.push(course);
                            }
                        }
                    }
                }
            }
            Ok(filtered_courses)
        }
        None => {
            return Ok(courses);
        }
    }
}

fn requirements_met(course: &Course, approved: &Vec<String>) -> bool {
    for requirement in &course.requirements {
        if !approved.contains(&requirement) {
            return false;
        }
    }
    return true;
}

pub fn get_courses_from_json(path: String) -> Result<Vec<Course>, error::Error> {
    let json = std::fs::read_to_string(&path)
        .map_err(|e| error::Error::CouldNotCreateFile(path.into(), e))?;
    let courses = serde_json::from_str(&json).map_err(|e| error::Error::JsonDeserialization(e))?;
    return Ok(courses);
}

pub fn approve_courses(courses: &Vec<String>) -> Result<(), error::Error> {
    let path = courses_files_path()?.join("approved.json");
    let json = std::fs::read_to_string(&path)
        .map_err(|e| error::Error::CouldNotOpenFile(path.clone(), e))?;
    let mut approved: Vec<String> =
        serde_json::from_str(&json).map_err(|e| error::Error::JsonDeserialization(e))?;
    for course in courses {
        if approved.contains(&course) {
            return Err(error::Error::CourseAlreadyApproved(course.to_string()));
        }
        // check that the course exists
        let courses = get_courses(None)?;
        if !courses.iter().any(|c| &c.code == course) {
            return Err(error::Error::CourseDoesNotExist(course.to_string()));
        }
        approved.push(course.to_string());
    }
    let json = serde_json::to_string(&approved).map_err(|e| error::Error::JsonSerialization(e))?;
    std::fs::write(&path, json).map_err(|e| error::Error::CouldNotCreateFile(path, e))?;
    Ok(())
}
