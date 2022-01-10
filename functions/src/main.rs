fn main() {
    
    param_func(40);

    let x = 5;
    // Bind a value using an expression
    let y = {
        let x = 3;
        // no ; since this is an expression not a statement
        x + 1
    };

    println!("Y = {}", y);

    println!("A five = {}", return_five());

    println!("Eleven = {}", plus_ten(1));

}

fn param_func(x: i32) {
    println!("Param value: {}", x);
}


// Return a 5
// No need to specify return, function returns last value implicitly
fn return_five() -> i32 {
    // no semicolon either
    5
}

fn plus_ten(x: i32) -> i32 {
    x + 10
}
