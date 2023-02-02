use tabled::Tabled;

#[derive(Tabled)]
pub struct CourseTable<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub status: &'a str,
}
