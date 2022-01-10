fn main() {
    // ======== V E C T O R S =========

    // Create a new Vector of integetrs
    let v: Vec<i32> = Vec::new();

    // Create new vector using initial values
    let v2 = vec![1, 2, 3, 4];

    // Initialize a mutable vector and add values to it
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(9);
    v3.push(99);
    v3.push(55);

    println!("{:?}", v3);

    // How to reference a value inside a vector
    let third_val: &i32 = &v3[2];
    println!("Third element: {}", third_val);

    // Second option
    let third_val_2 = v3.get(2);
    println!("{:?}", third_val_2); // returns a option<T> => Some(number)

    // Correct way of printing just the number
    let num_placeholder: i32;
    match &v3.get(2) {
        Some(third) => println!("Third value of match option: {}", third),
        None => println!("No third value found!"),
    }

    // Get rearurn an Option<&T> and returns a None if index does not exist
    let out_of_bounds = v3.get(100);
    println!("Index 100 = {:?}", out_of_bounds);

    // Iterating over a vector
    // gets immutable reference to each element
    for i in &v3 {
        println!("i: {}", i);
    }

    // Iterate as mutable reference
    let mut my_vec = vec![40, 50, 60, 70];

    for i in &mut my_vec {
        *i += 100;
        println!("{}", i);
    }

    // Using ENUMS to be able to hold different data types in same vector
    #[derive(Debug)]
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // The type that the vector hold is of type Enum Spreadsheet
    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("Blue")),
        Spreadsheet::Float(44.44),
    ];

    let mut test_row: Vec<Spreadsheet> = Vec::new();
    test_row.push(Spreadsheet::Float(5.0));
    test_row.push(Spreadsheet::Text(String::from("gaylord")));
    test_row.push(Spreadsheet::Text(String::from("gaylord2")));
    test_row.push(Spreadsheet::Int(88));
    println!("{:?}", test_row);

    // ====== S T R I N G S ======
    println!("=== STRINGS START HERE ===");

    // Create mutable string
    let mut s = String::new();

    // Initialize some data to put into our s variable
    let data = "Important content!";

    let s = data.to_string();

    // Equivalent as above:
    let s = String::from("This is the data: ") + data;
    println!("{}", s);
    println!("{}", data);

    // Adding text to string
    let mut foo = String::from("foo");
    foo.push_str("bar");
    println!("{}", foo);
    println!("{}", foo);
    let bar = "bar";
    foo.push_str(bar); // push_str is only borrowing the bar variable
    println!("We can still use bar: {}", bar);

    // If we only want to add a char, we use the push method
    foo.push('K');
    println!("{}", foo);

    // Concatenate two strings
    let str1 = String::from("Mother, ");
    let str2 = String::from("Can you tell your children not to...");
    let str3 = str1 + &str2; // str2 is borrowed, str1 not. Thus str1 can't be referred to after this
    println!("{}", str3);

    // String macro: format!
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let ttt = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", ttt);

    // S T R I N G  I N D E X I N G
    let str_i = "normal latin";
    // first (and most dangerous way) is using string slicing
    let slice1 = &str_i[0..1];
    println!("{}", slice1);

    // iterating trough all chars (now we don't need to know if the letter is a one or two byte UTF-8)
    for c in "pippeli".chars() {
        println!("{}", c);
    }
    // Same printing the byte representation of the char
    for c in "kuk".bytes() {
        println!("{}", c);
    }


    // ===== H A S H  M A P S =====
    use std::collections::HashMap;

    // Create new hash map
    let mut scores = HashMap::new();

    // Insert key-value pairs into the hash map
    scores.insert(String::from("BlueTeam"), 10);
    scores.insert(String::from("RedTeam"), 20);

    // Another way of constructing a hash map
    // First init a string vector
    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![100, 200];

    // Zip the vectors into a hash map using iter, zip and collect
    let new_scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    // Hash maps take ownership by default, if the value does not implement the copy trait
    let key_str = String::from("Homo?");
    let val_str = String::from("Todellakin!");
    let mut map = HashMap::new();
    map.insert(key_str, val_str);
    // cannot print the values after this, if not passing in vals as a reference

    // Getting values out of hash maps
    let my_key = String::from("Homo?");
    let is_homo = map.get(&my_key);
    println!("Onko homo?: {:?}", is_homo);

    match is_homo {
        Some(homo) => println!("Onse homo..."),
        None => println!("Ei löydy homoja täältä!"),
    }

    // Iterating trough key vals in hash map
    for (key, value) in &new_scores {
        println!("{}: {}", key, value);
    }

    // UPDATING A HASH MAP

    // Overwriting the old value is the default
    // Check if our hash map "map" has a key named "kuk", if not insert it
    map.entry(String::from("kuk?")).or_insert(String::from("Yes, please"));
    println!("{:?}", map);

    // Updating a value based on the old value
    let text =  "world full of world";

    let mut word_map = HashMap::new();

    // word counter
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0); // If it is the first time we see the word, count is 0
        *count += 1; // Count is of type (&mut V)
    }

    println!("{:?}", word_map);





}
