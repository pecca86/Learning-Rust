use std::collections::HashMap;

// A company with different appartments
// People can be added to an apartment
// We can list all people in an department in alphabetical order
fn main() {

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut people: Vec<String> = Vec::new();

    people.push("Knull-Karl".to_string());
    people.push("Fitt-Göran".to_string());

    company.insert(
        String::from("homot"),
        people,
    );

    println!("{:?}", company);

}

fn add_person_to_department() {
    //company.insert(department, persons.push(name);

}

fn list_people_of_department() {}
