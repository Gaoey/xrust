#[allow(dead_code)]
pub struct Employee<'a> {
    pub(crate) name: &'a str,
    pub(crate) position: &'a str,
    pub(crate) age: i32,
}

#[derive(PartialEq, Debug)]
pub struct People<'a> {
    pub(crate) name: &'a str,
    pub(crate) age: i32,
}

impl<'a> TryFrom<Employee<'a>> for People<'a> {
    type Error = String;

    fn try_from(value: Employee<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            age: value.age,
        })
    }
}

pub fn map_employee_to_people(emp: Employee) -> Result<People, String> {
    match emp.try_into() {
        Ok(p) => Ok(p),
        Err(_) => return Err("cannot convert type employee".to_string()),
    }
}
