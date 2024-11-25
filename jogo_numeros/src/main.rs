use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=10);

    //println!("Random number: {}", rand_num);

    loop {
        let mut guess = String::new();
        println!("Guess a number from 1 to 10: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
