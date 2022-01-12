use std::collections::HashMap;
use std::io;

// A company with different appartments
// People can be added to an apartment
// We can list all people in an department in alphabetical order
fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome! Please type 'add' then enter the name of the person, then the department to witch you would like to add the person\nIf you want to look up all the staff in a given department, type 'lookup' and then enter the department name");

    loop {
        println!("Type 'add' or 'lookup'");
        let mut search_option = String::new();

        // Take user input
        io::stdin()
            .read_line(&mut search_option)
            .expect("Failed to read line.");

        // Trim out trailing newline
        let search_option = search_option.trim();

        if search_option.eq(&"add".to_string()) {
            let mut person = String::new();
            let mut department = String::new();

            println!("\nPlease enter the name of the person: ");
            io::stdin()
                .read_line(&mut person)
                .expect("Failed to read line");
            let person = person.trim();

            println!("\nPlease enter the name of the department: ");
            io::stdin()
                .read_line(&mut department)
                .expect("Failed to read line");
            let department = department.trim();
            println!("Adding '{}' to {}...", person, department);

            add_person_to_department(&mut company, person.to_string(), department.to_string());
        } else if search_option.eq(&"lookup".to_string()) {
            let mut department = String::new();
            println!("\nPlease enter the name of the department: ");
            io::stdin()
                .read_line(&mut department)
                .expect("Failed to read line");
            let department = department.trim();

            list_people_of_department(&company, department.to_string());
        } else {
            println!("Invalid option");
        }
    }
}

// Add person to given department
// Company: HashMap
// Employee: String
// Department: String
fn add_person_to_department(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department: String,
) {
    let employees = company.get(&department);
    let mut staff: Vec<String> = Vec::new();

    match employees {
        Some(ppl) => staff = ppl.to_vec(),
        None => println!("Department does not exist, creating a new one..."),
    }

    // Push employee to the vector, then sort it alphabetically
    staff.push(employee);
    staff.sort_unstable();

    company.insert(department, staff);
}

// Print the staff of the given apartment
fn list_people_of_department(company: &HashMap<String, Vec<String>>, department: String) {
    let staff = company.get(&department);

    match staff {
        Some(employess) => println!(
            "The department of {} consists of the following: {:?}",
            department, employess
        ),
        None => println!("No employees for given department: {}", department),
    }
}
