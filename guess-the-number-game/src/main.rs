use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hi, I guessed a number. Let's see if you can find it.");

    //Random number between 1-10
    let number = rand::thread_rng().gen_range(1..10);

    //Endless game until you find the number :)
    loop {
        let mut guess = String::new();

        //Get input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Need to be parsed as integer. If it is not, print some message
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please write a number");
                continue;
            }
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("A little bit bigger ↑"),
            Ordering::Greater => println!("A little bit smaller ↓"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
