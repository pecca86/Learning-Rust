// Normal struct
struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}

// Normal struct with a tuple struct inside
struct AdvancedUser {
    name: String,
    favorite_color: Color
}

// Tuple struct
struct Color(i32, i32, i32);


fn main() {
    let mut user1 = User {
        username: String::from("Håmå-Olle"),
        email: String::from("h-o@gmail.com"),
        sing_in_count: 0,
        active: true
    };

    println!("Username: {}", user1.username);
    user1.username = String::from("Keiman");
    println!("Username: {}", user1.username);

    let second_user = create_user(String::from("Fitt-Patrik"), String::from("knulla@ireven.fi"));
    println!("{}", second_user.username);
    
    // Create a new user an copy values from previous user into data fields
    let third_user = User {
        username: String::from("Thrid hole"),
        ..second_user
    };
    println!("{}, {}", third_user.username, third_user.email);

    // === TUPLE STRUCT ===
    
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(1,23,4);

    let adv_user = AdvancedUser {
        name: String::from("kukukuk"),
        favorite_color: black
    };

    println!("{} - {}", adv_user.name, adv_user.favorite_color.1);
}


// Create a new user
fn create_user(name: String, email: String) -> User {
    User {
        username: name,
        email,
        sing_in_count: 1,
        active: true,
    }
}