mod back_of_house;
mod front_of_house;

pub mod customer {
    use crate::back_of_house::{Appetizer, Breakfast};
    use crate::front_of_house::hosting::*;

    pub fn eat_at_restaurant() {
        add_to_waitlist();

        let mut meal = Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
        println!("Also order {order1:?} and {order2:?} appetizers.");
    }
}
