use rand::Rng;


pub struct Lib;

impl Lib{
    pub fn random_min_max(min: f32, max: f32) -> f32 {
        if min < max{
            return min + (max - min) * Lib::random_double()
        }
        else{
            panic!("max is less than min")
        }
    }
    pub fn random_double() ->f32
    {
        rand::thread_rng().gen_range(0.0,1.0)
    }
}