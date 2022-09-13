use utils::check_type;
use utils::mars::{read_weight_on_earth, calculate_weight_on_mars};

mod utils;

fn main() {
    println!("Enter weight at earth: ");
    let weight_on_earth :f32 = read_weight_on_earth();
    let weight_on_mars :f32 = calculate_weight_on_mars(weight_on_earth);
    println!("The weight in mars is {:.2}", weight_on_mars);
    println!("var type: {}", check_type(weight_on_mars));
    println!("var type: {}", check_type::<i32>(-800));
    println!("var type: {}", check_type("var"));
    println!("var type: {}", crate::utils::check_type([1]));

}
