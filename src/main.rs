use utils::{Person, File, GetAdditionalInfo};
use utils::check_type;
use utils::mars::{read_weight_on_earth, calculate_weight_on_mars};

mod utils;

fn main() {

    let person = Person::new(String::from("Fred"), 35, String::from("Bololo"));
    let file = File::new(String::from(r#"C:\file.txt"#), 1024, String::from("Empty text file"));

    println!("Person name is: {} and age is: {}", person.get_name(), person.get_age());
    println!("Person additional info is: {}", person.get_additional_info());
    println!("Person additional info type: {}", person.get_additional_info_type());

    println!("File path is: {}", file.get_path());
    println!("File size is: {}", file.get_size());
    println!("File additional infor is: {}", file.get_additional_info());

    println!("Enter weight at earth: ");
    let weight_on_earth :f32 = read_weight_on_earth();
    let weight_on_mars :f32 = calculate_weight_on_mars(weight_on_earth);
    println!("The weight in mars is {:.2}", weight_on_mars);
    println!("var type: {}", check_type(weight_on_mars));
    println!("var type: {}", check_type::<i32>(-800));
    println!("var type: {}", check_type("var"));
    println!("var type: {}", crate::utils::check_type([1]));

}
