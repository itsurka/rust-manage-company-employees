# Manage company employees

Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

I made this application during study the book - [The Rust Programming Language](https://doc.rust-lang.org/book)

## Build and run
```shell
cargo run
```

Example: 

```text
% cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/rust-manage-company-employees`

1: View all departments
2: View department
3: Add employee
q: Quit
3
Insert command like "Add Sally to Engineering" or "m" to return back:
Add Igor to Engineers

1: View all departments
2: View department
3: Add employee
q: Quit
3
Insert command like "Add Sally to Engineering" or "m" to return back:
Add Maria to QA

1: View all departments
2: View department
3: Add employee
q: Quit
3
Insert command like "Add Sally to Engineering" or "m" to return back:
Add Ion to Engineers

1: View all departments
2: View department
3: Add employee
q: Quit
1
- Engineers:
    - Ion
    - Igor
- QA:
    - Maria

1: View all departments
2: View department
3: Add employee
q: Quit
2
Insert department name or "m" to go back:
QA
- QA:
    - Maria

1: View all departments
2: View department
3: Add employee
q: Quit
3
Insert command like "Add Sally to Engineering" or "m" to return back:
m

1: View all departments
2: View department
3: Add employee
q: Quit
q
```