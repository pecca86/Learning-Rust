#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



// == USE KEYWORD ==
// Using 'use' we bring the function in to scope so we don't always have to write the whole path to it
// Absolute path
// Relative path and re-exporting it by making it public
pub use self::front_house::serving;


// Customer side of restaurant, functionality in different file which is shown by ending module with ;
mod front_house;
pub use crate::front_house::hosting;

fn serve_fixed_order() { println!("Fixed your food, hoe!")}

// Worker side of the restaurant
mod back_house {

    // Creating a public struct, with some public values
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // This is private by default
    }

    // Methods for our Breakfast struct
    impl Breakfast {
        // Creates a toast with summer season fruit
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    // Public enums and all its values are by default also public
    pub enum Appetizer {
        Soup,
        Salad,
    }


    fn fix_order() {
        cook_order();
        // Creates a relative path to the function serve_order() inside front house
        super::serve_fixed_order();
    }

    fn cook_order() {}
}


// Define a public function with a absolute and relative path to our front_house modules
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_house::hosting::add_to_waitlist();

    // Relative path
    front_house::hosting::add_to_waitlist();

    // Test our breakfast struct
    let mut meal = back_house::Breakfast::summer("Knaeckebroed");
    // Change mind about bread
    meal.toast = String::from("Skaergaordsbroed");
    println!("I'm ordering a {} toast please", meal.toast);
    // We can't change the fruit like this though, since it's private
    // meal.seasonal_fruit = String::from("Shit-berries");

    // Using our public enum
    let order1 = back_house::Appetizer::Salad;
    let order2 = back_house::Appetizer::Soup;
}


// Now our function can use it
pub fn eat_me_out() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}