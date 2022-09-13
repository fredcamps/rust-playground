use std::io;

pub fn check_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

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
