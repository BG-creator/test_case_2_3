
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    instructions();

    loop {
        print!("Enter command:");
        io::stdout().flush().unwrap();
        let command = get_input();

        if command.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }
        process_command(&command, &mut departments);
    }

}

fn instructions() {

    println!("###### USER INTERFACE #######");
    println!("## AVAILABLE COMMANDS : ##");
    println!("Add <Employee> to <department>");
    println!("List <department>");
    println!("List All");
    println!("Exit");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn process_command(command: &str, departaments: &mut HashMap<String, Vec<String>>) {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return;
    }
    match tokens[0].to_lowercase().as_str() {
        "add" => user_add(&tokens, departaments),
        "list" => list_all(&tokens, departaments),
        _ => println!("Invalid command"),
    }
}
fn user_add(tokens: &Vec<&str>, departments: &mut HashMap<String, Vec<String>>) {
    if tokens.len() < 4 {
        println!("Invalid format command");
        return;
    }
    if tokens[2].to_lowercase() != "to" {
        println!("Invalid format use : Add <Employee> to <Department>");
        return;
    }

    let  employee = tokens[1].to_string();
    let department = tokens[3].to_string();
    departments
        .entry(department.clone())
        .or_insert_with(Vec::new)
        .push(employee.clone());
    println!("Employee added '{}' to department '{}'", employee, department);

}

fn list_all(tokens: &Vec<&str>, departments: &mut HashMap<String, Vec<String>>) {

    if tokens.len() < 2 {
        println!("Invalid format command, use: List <Department> or List all");
        return;
    }
    if tokens[1].to_lowercase() == "all" {
        let mut departament: Vec<String> = departments.keys().cloned().collect();
        departament.sort();
        for d in departament {
            println!("{}", d);
            let mut employees = departments.get(&d).unwrap().clone();
            employees.sort();
            for e in employees {
                println!("- {}", e);
            }
        }
    }
    else{
        let department = tokens[1].to_string();
        match departments.get(&department) {
            Some(employess) => {
                let mut sorted_employees = employess.clone();
                sorted_employees.sort();
                println!("Department: '{}'", department);
                for e in sorted_employees {
                    println!("- {}", e);
                }
                       }
            None => println!("Department '{}' not found", department),
        }
    }

}