#[derive(Debug)]
enum Coin {
    // Put another enum inside the enum value
    Mark(CoinPerson),
    Euro,
    Cent,
    Penni,
}

#[derive(Debug)]
enum CoinPerson {
    Kekkonen,
    Koivisto,
    Ahtisaari,
    Ranta_aho(String),
}


impl Coin {
    fn print_pretty(&self) {
        println!("{:?} €", self);
    }
}

// Main program
fn main() {
    
    let c = Coin::Cent;
    value_in_cents(&c);
    c.print_pretty();

    // Enum with enum as a value
    let mark = Coin::Mark(CoinPerson::Ranta_aho(String::from("Ranta-aho")));
    value_in_cents(&mark);
    
}

// Value in cents
fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Mark(person) => {
            println!("The person on the coin is president {:?}", person);
            100
        }
        Coin::Euro => 1000,
        Coin::Cent => {
            println!("THIS IS A CENT YOU DUMBFUCK!");
            1
        }
        Coin::Penni => 10,
    }
}
