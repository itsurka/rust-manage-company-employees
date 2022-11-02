use crate::company::{Company, Department, Employee};
use crate::helper;

pub fn view_add_employee_menu(company: &mut Company) {
    let re = regex::Regex::new(r"^Add ([\w\s]+) to ([\w\s]+)").unwrap();
    let mut department: String = String::new();
    let mut employee: String = String::new();

    loop {
        println!("Insert command like \"Add Sally to Engineering\" or \"m\" to return back:");
        let mut data: String = String::new();
        let res = std::io::stdin()
            .read_line(&mut data);
        data = data.trim_end().to_string();

        // empty line, redraw the menu
        if res.is_err() {
            continue;
        }

        // "m" - menu option
        if data == "m" {
            return;
        }

        let caps_opt = re.captures(&data);
        if caps_opt.is_none() {
            continue;
        }

        let caps = caps_opt.unwrap();
        employee = caps.get(1).unwrap().as_str().to_string();
        department = caps.get(2).unwrap().as_str().to_string();

        break;
    }

    add_employee(company, employee, department);
}

// View department data by name
pub fn view_department_menu(company: &Company) {
    loop {
        println!("Insert department name or \"m\" to go back:");
        let mut data: String = String::new();
        let res = std::io::stdin()
            .read_line(&mut data);
        let cmd = data.trim();

        // empty line, redraw the menu
        if res.is_err() {
            continue;
        }

        // "m" - menu option
        if cmd == "m" {
            break;
        }

        let department_id = helper::to_snake_case(&cmd.to_string());
        let dep_opt = company.departments.get(&department_id as &str);

        if dep_opt.is_some() {
            view_department(company, &dep_opt.unwrap());
            break;
        }

        continue;
    }
}

// Show all departments with employees
pub fn view_departments(company: &Company) {
    for (_, department) in &company.departments {
        view_department(&company, &department);
    }
}

// Show department name and all employee's names in it
fn view_department(company: &Company, department: &Department) {
    println!("- {}:", department.name);
    let employees = company.department_employees.get(helper::to_snake_case(&department.name).as_str()).unwrap();
    for (_, employee_id) in employees {
        println!("    - {}", company.employees.get(*employee_id).unwrap().name);
    }
}

fn add_employee(company: &mut Company, employee: String, department: String) {
    let employee_id = company.add_employee(employee);
    let department_id = company.add_department(department);
    company.add_employee_to_department(employee_id, department_id);
}
