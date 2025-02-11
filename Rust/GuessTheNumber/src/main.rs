use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(" ***Guess the number");
    let secret_num = rand::thread_rng().gen_range(1..=100); //Might need to explore a new method

    loop {
        println!("Enter your guess (1..100)");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low. Try again!"),
            Ordering::Greater => println!("Too high. Try again!"),
            Ordering::Equal => {
                println!("Correct!! YAY");
                break;
            }
        }

    }


}
