use rand::Rng;

pub struct Randomizer {
    pub secret: i32,
}

impl Randomizer {
    pub fn generate() -> Self {
        println!("Загадай число!");
        let r = Self {
            secret: rand::thread_rng().gen_range(1..101),
        };
        println!("Загаданное число: {}", &r.secret);
        r
    }
}