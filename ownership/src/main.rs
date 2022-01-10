fn main() {
    // String from a string literal
    let mut s = String::from("Pekka");
    // Append text to string
    s.push_str(" Rules!");
    println!("{}", s);

    // This won't work, since string literal
    // let mut text = "Test";
    // text.push_str("Scmest");

    // Assigning s to another variable name makes the previous unavailable
    let s2 = s; // new owner of memory space
    println!("{}", s2);
    //println!("{}", s); <<< NOT LONGER AVAILABLE TO THE COMPLIER

    // Deep copy aka Clone
    let s3 = String::from("Homo");
    let s4 = s3.clone();

    println!("s3: {}, s4: {}", s3, s4);

    let s5 = String::from("Kekka");
    take_ownrshp(s5);

    // This results in an error, since s5 is now out of scope and String does not implement copy trait
    //println!("{}", s5);
    
    let x = 5;
    make_cp(x);
    // Makes a deep copy since integers implement the copy trait
    println!("{}", x);


    // USING REFERENCES
    let my_str = String::from("Howdy");
    let len = calculate_str_len(&my_str);
    println!("The length of my_str is: {}", len);

    // References are immutable by default
    // If we need to change someting we need to declare it as a mut
    let mut mutable_str = String::from("Kakka");
    change_mutable_string(&mut mutable_str);
    println!("New string: {}", mutable_str);

    let r1 = &mut mutable_str;
    println!("r1: {}", r1);
    {
        let r2 = &mut mutable_str;
        println!("r2: {}", r2);
    }
    //println!("r1: {}", r2); // not valid since r2 is out of scope
    

    // ==== USING SLICES ====
    let mut slice_str = String::from("Slice me up!");
    let first_word = first_word(&slice_str);
    println!("First word of string is: {}", first_word);
    slice_str.clear();
    //println!("First word of string is: {}", first_word); // compiler won't let us use first_word anymore since orignal string is altered!


}

// Find first word in string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// Change a mutable string
fn change_mutable_string(s: &mut String) {
    s.push_str(" maistuu aina!");
}

// Calculated length of immutable string using reference
fn calculate_str_len(s: &str) -> usize {
    s.len()
}


fn take_ownrshp(s: String) {
    println!("The string is owned! {}", s);
}

fn make_cp(num: i32) {
    println!("I don't own shit!{}", num);
}