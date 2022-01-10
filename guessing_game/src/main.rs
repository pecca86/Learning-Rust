use std::io;

fn hint(guess: &str, password: &String ) {

    if guess.len() < password.len() {
        println!("Liian lyhyt salasana");
        return;
    }

    if guess.len() > password.len() {
        println!("Liian pitkä salansana!");
        return;
    }

    for n in 0..password.len() {

        let pwch = password.chars().nth(n).unwrap();
        let guessch = guess.chars().nth(n).unwrap();

        if pwch.eq(&guessch) {
            println!("Indeksi {} kirjain '{}' oli oikein...", n, pwch);
        }
    }

    println!("Salasanan pituus oikea...");
}

fn main() {
    println!("Arva oikea salasana (vihje: pienillä kirjaimilla)!");

    let salasana: String = String::from("janne");

    loop {
        println!("Salansana:");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read input");

        let guess = guess.trim();
    
        println!("Arvasit: {}", guess);
    
        if guess.eq(&salasana) {
            println!("Oikein!");
            break;
        } else {
            hint(guess, &salasana);
        }

    }
    println!("Bitlocker salasana on: Toroskainen<3Piispa");
}
