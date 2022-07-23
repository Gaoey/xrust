#[cfg(test)]
mod test {
    use crate::techniques::type_conversion::type_conversion::{
        map_employee_to_people, Employee, People,
    };

    #[test]
    fn test_type_conversion() {
        let employee1 = Employee {
            name: "Gaoey",
            position: "Programmer",
            age: 30,
        };

        let people1 = map_employee_to_people(employee1);

        match people1 {
            Ok(p) => assert_eq!(
                p,
                People {
                    name: "Gaoey",
                    age: 30
                }
            ),
            Err(_) => println!("cannot convert type employee"),
        }
    }
}
