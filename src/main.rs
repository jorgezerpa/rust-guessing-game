use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("{}","Guess the number!".purple());

    let secret_number:i32 = rand::thread_rng().gen_range(1,100);

    loop {
        println!("Please input your guess:");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {}", guess);
    
        let guess:i32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_)=>{
                println!("Invalid input!");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!!".blue());
                break;
            }
        }
    }

}
