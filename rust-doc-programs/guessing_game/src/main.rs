use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Passing in a mutable string.
                                   // read_line() function will append the input to this variable.
                                   // Note: `guess` is NOT "returned", it's only altered/appended to.
                                   //       `guess` remains in scope during this call.
                                   //
                                   //        A `Result` type is returned and contains the `.expect()`
                                   //        helper method.
            .expect("Failed to read line");

        // Shadowing!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
