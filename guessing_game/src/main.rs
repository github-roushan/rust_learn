use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("{}", "## Guess my number program ##".bold().cyan());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("{}", "Input your guess: ".yellow());
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the variable guess");

        let guess = guess.trim().parse::<u32>();
        let guess = match guess {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Only Valid Inputs are Integers".red());
                continue;
            }
        };

        println!(
            "{} {}",
            "You guessed:".blue(),
            format!("{guess}").bold().magenta()
        );

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small\n".red()),
            Ordering::Greater => println!("{}", "Too big\n".red()),
            Ordering::Equal => {
                println!("{}", "You win!\n".green().bold());
                break;
            }
        }
    }
}
