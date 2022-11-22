pub mod error;
pub mod randomizer;

use error::Error;
use randomizer::Randomizer;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::io::{self, Write};

pub trait Logic {
    fn new() -> Self;
    fn start(&self);
}

pub struct Game {
    pub secret: i32,
    pub players_count: RefCell<i32>,
}

impl Game {
    fn stdin_players_num() -> Result<i32, Error> {
        print!("Введи число игроков: ");
        std::io::stdout().flush()?;
        let mut players = String::new();
        io::stdin().read_line(&mut players)?;
        return match players.trim().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => Err(Error::from(e)),
        };
    }

    fn stdin_player_answer(player: &i32) -> Result<i32, Error> {
        println!("Догадались какое число загадано игрок {}?", player);
        print!("Загаданное число: ");
        std::io::stdout().flush()?;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        return match guess.trim().parse() {
            Ok(num) => {
                println!("{}-й игрок ваша догадка: {}", player, num);
                Ok(num)
            }
            Err(e) => Err(Error::from(e)),
        };
    }

    fn is_equal(&self, guess: &i32, player: &i32) -> bool {
        let mut is_eq = false;
        match guess.cmp(&self.secret) {
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => {
                println!("Вы выиграли игрок {}!", player);
                is_eq = true;
            }
        };
        is_eq
    }
}

impl Logic for Game {
    fn new() -> Self {
        let random = Randomizer::generate();
        Self { 
            players_count: RefCell::new(0),
            secret: random.secret,
        }
    }

    fn start(&self) {
        let mut i = 1;

        loop {
            if *self.players_count.borrow() == 0 {
                if let Ok(players_count) = Game::stdin_players_num() {
                    self.players_count.replace(players_count);
                }
            } else {
                if i <= *self.players_count.borrow() {
                    if let Ok(guess) = Game::stdin_player_answer(&i) {
                        if self.is_equal(&guess, &i) {
                            break;
                        }
                        i += 1;
                    } else {
                        continue;
                    };
                } else {
                    i = 1;
                }
            };
        }
    }
}
// pub trait Logic {
//     fn guess_number(&self) {
//         println!("Загадай число!");
//         let secret_number = rand::thread_rng().gen_range(1..101);
//         println!("Загаданное число: {}", secret_number);

//         let mut i_numbers = 0;
//         loop {
//             println!("Введи число игроков");
//             let mut numbers = String::new();
        
//             io::stdin()
//                 .read_line(&mut numbers)
//                 .expect("Failed to read line");
//             let numbers: i32 = match numbers.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => -1,
//             };
    
//             if numbers == -1 {
//                 println!("Неправильное число игроков");
//             } else {
//                 i_numbers = numbers;
//                 break;
//             }
//         }

//         let mut i = 1;
//         loop {
//             if i <= i_numbers {
//                 println!("Догадались какое число загадано игрок {}?", i);
//                 let mut guess = String::new();
//                 io::stdin()
//                     .read_line(&mut guess)
//                     .expect("Failed to read line");
//                 let guess: u32 = match guess.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => continue,
//                 };
//                 println!("{}-й игрок введите вашу догадку: {}", i, guess);
//                 match guess.cmp(&secret_number) {
//                     Ordering::Less => println!("Слишком мало!"),
//                     Ordering::Greater => println!("Слишком много!"),
//                     Ordering::Equal => {
//                         println!("Вы выиграли игрок {}!", i);
//                         break;
//                     }
//                 }
//                 i += 1;
//             } else {
//                 i = 1;
//             }
//         }
//     }
// }

// pub struct Game;
//     // number_game: i32,
//     // number_players: i32,

// impl Logic for Game {}







