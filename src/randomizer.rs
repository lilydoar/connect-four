use raylib::get_random_value;

pub trait Randomizer {
    fn random_value(&mut self, min: i32, max: i32) -> i32;
}

pub struct RaylibRandomizer;

impl Randomizer for RaylibRandomizer {
    fn random_value(&mut self, min: i32, max: i32) -> i32 {
        get_random_value(min, max)
    }
}
