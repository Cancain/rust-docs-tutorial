//mod front_of_house {
    //pub mod hosting {
        //pub fn add_to_waitlist() {}
    //}
//}

//fn serve_order() {}

//mod back_of_house {
    //fn fix_incorrect_order() {
        //cook_order();
        //super::serve_order();
    //}

    //fn cook_order() {}
//}

//pub fn eat_at_resturant() {
    ////Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    ////Relative path
    //front_of_house::hosting::add_to_waitlist();
    
//}

////Puiblic structs
//mod back_of_house {
    //pub struct Breakfast {
        //pub toast: String,
        //seasonal_fruit: String,
    //}

    //impl Breakfast {
        //pub fn summer (toast: &str) -> Breakfast {
            //Breakfast {
                //toast: String::from(toast),
                //seasonal_fruit: String::from("Peaches"),
            //}
        //}
    //}
//}

//pub fn eat_at_resturant() {
    //let mut meal = back_of_house::Breakfast::summer("Rye");

    //meal.toast = String::from("Wheat");
    
    //println!("I'd like {} toast please", meal.toast);
//}

//Public enums
mod back_of_house() {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_resturant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}