use std::io;

pub struct Person<T> {
   name: String,
   age: u8,
   additional_info: T,
}

pub struct File<T> {
    path: String,
    size: u32,
    additional_info: T,
}

pub trait GetAdditionalInfo<T> {
    fn get_additional_info(&self) -> &T;
    fn get_additional_info_type(&self) -> &'static str;
}

#[macro_export]
macro_rules! get_additional_info_impl_for_struct {
    ($struct_name: ident) => {
        impl <T> GetAdditionalInfo<T> for $struct_name<T> {
            fn get_additional_info(&self) -> &T {
                &self.additional_info
            }

            fn get_additional_info_type(&self) -> &'static str {
                check_type(&self.additional_info)
            }

        }
    };
}

impl<T> Person<T> {
    pub fn new(name: String, age: u8, additional_info: T) -> Self {
        get_additional_info_impl_for_struct!(Person);
        Person { name, age, additional_info }
    }

    pub fn get_age(&self) -> &u8 {
        &self.age
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl<T> File<T> {
    pub fn new(path: String, size: u32, additional_info: T) -> Self {
        get_additional_info_impl_for_struct!(File);
        File { path, size, additional_info }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_size(&self) -> &u32 {
        &self.size
    }
}

pub fn check_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

// pub fn instanceof<T>(type1: T, type2: T) -> bool {
//     check_type(type1) == check_type(type2)
// }

pub mod mars {

    const MARS_WEIGHT_DIVIDER: f32 = 9.81;
    const MARS_WEIGHT_MULTIPLIER: f32 = 3.711;

    pub fn read_weight_on_earth() -> f32 {
        let mut buffer = String::new();
        super::io::stdin().read_line(&mut buffer).expect("Enter a valid number!");
        buffer.trim().parse().expect("Enter a valid number !")
    }

    pub fn calculate_weight_on_mars(weight: f32) -> f32 {
        (weight / self::MARS_WEIGHT_DIVIDER) * self::MARS_WEIGHT_MULTIPLIER
    }

}
