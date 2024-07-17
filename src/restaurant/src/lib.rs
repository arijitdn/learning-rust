mod front_of_house;

mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// use crate::front_of_house::hosting; // Now that this module is in scope, we can specify it more easily
pub use self::front_of_house::hosting; // Adding a pub here allows other files to use this

pub fn eat_at_restaurant() {
    // Absolute Path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;
}

fn serve_order() {}