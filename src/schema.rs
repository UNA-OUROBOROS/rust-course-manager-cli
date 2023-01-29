// @generated automatically by Diesel CLI.

diesel::table! {
    course_requirements (course_code, requirement_code) {
        course_code -> Nullable<Text>,
        requirement_code -> Nullable<Text>,
    }
}

diesel::table! {
    courses (code) {
        code -> Text,
        name -> Text,
        credits -> Integer,
        year -> Integer,
        semester -> Integer,
        is_bachelor -> Integer,
        is_completed -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    course_requirements,
    courses,
);
