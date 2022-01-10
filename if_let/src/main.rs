#[derive(Debug)]
enum Coin {
    Quater(String),
    Euro,
}

fn main() {
    // if let conditional
    let some_u8_val = Some(0u8);
    let mut count = 0;

    // This is the same as the my_euro match example
    if let Some(3) = some_u8_val {
        println!("FOUND A THREE!");
    } else {
        count += 1;
    }

    // Demonstrating how match works as a conditional
    let my_coin = Coin::Quater(String::from("penni"));

    println!("Count: {}", count);
    
    match my_coin {
        Coin::Quater(fin) => println!("A finnish penni: {:?}", fin),
        _ => count +=1,
    }

    println!("Count: {}", count);
    
    let my_euro = Coin::Euro;
    
    match my_euro {
        Coin::Quater(fin) => println!("{}", fin),
        _ => count += 1,
    }

    println!("Count: {}", count);
}
