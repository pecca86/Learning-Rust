fn main() {

    // Using the enum option
    // In the background it looks like:
    // enum Option<T> { Some(T), None, }
    let some_num = Some(5);
    let some_str = Some("Bitches be strings");

    // A null value in Rust
    let absent_num: Option<i32> = None;

    println!("{:?}", add_one(&some_num));
    println!("{:?}", add_one(&absent_num));

    let uno = 1;
    let dos = 2;
    print_number_to_letters(uno);
    print_number_to_letters(dos);

}

// Using match arms
fn add_one(number: &Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// Using the _ placeholder to match any that we are not interested in but still need to cover
// Since match is exhaustive!
fn print_number_to_letters(num: i32) {
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    }
}
