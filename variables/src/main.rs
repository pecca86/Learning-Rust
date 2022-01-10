
const BUFFER_MAX: u32 = 100;

fn main() {
    let mut x = 5;
    println!("X is {}", x);
    
    x = 6;
    println!("X is {}", x);

    for n in 0..BUFFER_MAX {
        if n%10 == 0 {
            println!("{}", n);
        }
    }

    // Shadowing requiers using 'let' once more
    let y = 10;
    let y = y + 11;
    println!("Y is {}", y);

    let mut _spaces = "     ";
    let _spaces = _spaces.len();

    let my_num: u8 = u8::MAX;
    let n: isize = 40;
    println!("Max unsigned 8-bit values is {}", my_num);
    println!("isize n{}", n);

    let floater: f64 = 0.00023;
    let is_gay: bool;

    if (4*4) == 16 {
        is_gay = true;
    } else {
        is_gay = false;
    }
    println!("Is Kalle gay? Answer: {}", is_gay);

    let my_char: char = '\n';
    println!("{} N {} K", floater, my_char);

    // TUPLES
    let tup: (i32, f64, char) = (500, 5.4, 'A');
    println!("{:?}", tup);
    let x = tup.0;

    println!("The first value in tup is {}", x);

    let (a, b, c) = tup;
    println!("All vals: {} {} {}", a, b, c);

    // ARRAYS
    let my_array: [i32; 5] = [1,2,34,5,6];
    
    let five_threes = [3; 5]; // 3, 3, 3, 3, 3
    

}
