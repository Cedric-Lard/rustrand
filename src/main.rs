use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main (){
    println!("Welcome To The Number Guessing Game!");
    println!("Please enter a number between 1-100 as a guess: ");

    let target = rand::thread_rng()
        .gen_range(1..=100);
    
    loop{
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
            println!("you guessed : {}", guess);

        let guess: u32 = match guess.trim()
            .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&target){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
