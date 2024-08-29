use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // rand::thread_rng() will take a number from current thread
    // and .gen_range() will return a number within a specif range, defined by "start..=end"

    loop { 
        println!("Please input your guess.");

        let mut guess: String = String::new(); // a string variable to store users input
        // usually, in rust, ::new() introduces a empty type variable

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // in case read_line returns an Err, expect will crash the program and output this error message
            // (note: get a warning without '.expect()' method)

        let guess: u32 = match guess // shadowing the variable guess
            .trim() // method that erases spaces from begin and end, and \n \r\n too
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            }; // match requires this structure

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Lower!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }, // match requires this structure
        }
    }
}
