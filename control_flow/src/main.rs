fn main() {
    let num = 7;

    if num < 5 {
        println!("Num is lesser than 5");
    } else if num % 2 == 0 {
        println!("Even number!")
    } else {
        println!("num is gt 5");
    }

    // Let condition
    let cond = true;
    // Expression values must match!
    let number = if cond {
        5
    } else {
        6
    };
    println!("Num is {}", number);
}
