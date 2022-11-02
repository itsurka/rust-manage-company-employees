mod company;
mod menu;
mod helper;

use std::collections::HashMap;
use crate::company::{Company, Department, Employee};
use crate::menu::{view_departments, view_department_menu, view_add_employee_menu};

const CMD_ALL_DEPARTMENTS: &str = "1";
const CMD_VIEW_DEPARTMENT: &str = "2";
const CMD_ADD_EMPLOYEE: &str = "3";
const CMD_QUIT: &str = "q";


fn main() {
    let departments: HashMap<String, Department> = HashMap::new();
    let employees: Vec<Employee> = Vec::new();
    let department_employees: HashMap<String, HashMap<usize, usize>> = HashMap::new();
    let mut company = Company {
        departments,
        employees,
        department_employees,
    };

    main_menu(&mut company);
}

fn main_menu(company: &mut Company) {
    loop {
        println!("\n{}: View all departments\n{}: View department\n{}: Add employee\nq: Quit", CMD_ALL_DEPARTMENTS, CMD_VIEW_DEPARTMENT, CMD_ADD_EMPLOYEE);

        let mut cmd: String = String::new();
        let res = std::io::stdin()
            .read_line(&mut cmd);
        cmd = cmd.trim_end().to_string();
        if res.is_err() {
            continue;
        }

        match cmd.as_str() {
            CMD_ALL_DEPARTMENTS => view_departments(company),
            CMD_VIEW_DEPARTMENT => view_department_menu(company),
            CMD_ADD_EMPLOYEE => view_add_employee_menu(company),
            CMD_QUIT => break,
            _ => continue,
        }
    }
}

