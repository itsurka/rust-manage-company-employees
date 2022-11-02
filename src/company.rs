use std::collections::HashMap;
use crate::helper;

pub struct Employee {
    pub name: String,
}

pub struct Department {
    pub name: String,
}

pub struct Company {
    pub departments: HashMap<String, Department>,
    pub employees: Vec<Employee>,
    pub department_employees: HashMap<String, HashMap<usize, usize>>,
}

impl Company {
    pub fn add_department(&mut self, name: String) -> String {
        let department_id = helper::to_snake_case(&name);
        let department = Department {
            name,
        };
        self.departments.entry(department_id.clone()).or_insert(department);

        department_id
    }

    pub fn add_employee(&mut self, name: String) -> usize {
        let employee = Employee {
            name,
        };
        self.employees.push(employee);

        self.employees.len() - 1
    }

    pub fn add_employee_to_department(&mut self, employee_id: usize, department_id: String) {
        let emp_map_opt = self.department_employees.get(department_id.as_str());
        let emp_map: HashMap<usize, usize>;
        match emp_map_opt {
            None => {
                emp_map = HashMap::new();
                self.department_employees.insert(department_id.clone(), emp_map);
            },
            _ => {},
        }

        self.department_employees.get_mut(department_id.clone().as_str()).unwrap().entry(employee_id).or_insert(employee_id);
    }
}