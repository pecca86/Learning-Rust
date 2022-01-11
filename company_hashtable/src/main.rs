use std::collections::HashMap;

// A company with different appartments
// People can be added to an apartment
// We can list all people in an department in alphabetical order
fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut people: Vec<String> = Vec::new();

    people.push("Knull-Karl".to_string());
    people.push("Fitt-Göran".to_string());

    company.insert(String::from("homot"), people);

    println!("{:?}", company);
    add_person_to_department(&mut company, "Pekka".to_string(), "homot".to_string())
}

fn add_person_to_department(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department: String,
) {

    let mut people: Vec<String> = Vec::new();

    people.push("Knull-Karl".to_string());
    people.push("Fitt-Göran".to_string());

    company.insert(String::from("homot"), people);


    let mut employees = company.get(&department);
    let mut vec: Vec<String> = Vec::new();
    match employees {
        Some(ppl) => vec = ppl.to_vec(),
        None => println!("ERROR!"),
    }
    println!("{:?}", employees);
    //employees.push(employee);
    //employees.sort_unstable();
//
    //company.insert(department, employees);
}

fn list_people_of_department() {}
