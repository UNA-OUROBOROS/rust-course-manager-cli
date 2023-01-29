create table courses(
    code         text not null
        primary key,
    name         TEXT not null,
    credits      integer not null,
    year         integer not null,
    semester     integer not null,
    is_bachelor  integer not null,
    is_completed integer not null
);

create table course_requirements
(
    course_code      text
        constraint course_requirements_courses_code_fk
            references courses,
    requirement_code text
        constraint course_requirements_courses_code_fk_2
            references courses,
    constraint course_requirements_pk
        primary key (course_code, requirement_code)
);