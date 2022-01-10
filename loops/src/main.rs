fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            // breaks and returns counter value
            break counter;
        }
    };

    println!("Result after loop: {}", counter);
    

    // While loop
    while counter != 0 {
        counter -= 1;
    }

    println!("Counter is ZERO: {}", counter);

    // FOR loop
    let arr = [10,20,30,40,50];

    for num in arr.iter() {
        println!("{}", num);
    }

    // FOR using RANGE and reverse order = rev()
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFOFF!");
}
