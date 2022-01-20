use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the numbah!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries: u32 = 0;
    
    loop {
        tries += 1; // increase loop counter

        let mut guess = String::new(); // if outside loop, stores prior inputs
        println!("Input now ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    println!("The secret was {}", secret_number);
    println!("Number of tries: {}", tries)
}
