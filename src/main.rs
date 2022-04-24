use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    /*
     * Generate a random u32 integer between 0 and 100.
     *
     * Sets as immutable secret_number variable.
     */
    let secret_number = rand::thread_rng().gen_range(0, 101);

    // Declares beginning of the game.
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // Creating guess as mutable String variable.
        let mut guess = String::new();

        // Taking in standard input and appending it to the
        // guess String, taking it in as a mutable reference.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
         * Shadowing guess by defining a new guess which is u32.
         * Trim and parse the String guess into u32 integer.
         * Match with two branches of parse (Ok and Err).
         * Will start loop over if Err.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Printing out user guess.
        println!("You guessed: {}", guess);

        /*
         * Comparing guess to secret number (immutable reference).
         *
         * Matches against three branches of cmp (which returns an Odering enum).
         *
         * If the guess is too small or too big, it will say so and redo the loop.
         *
         * If the guess is equal to the secret number, then it will declare that you have won
         * and then exit the loop.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // On equal branch, print that user has one and break from loop.
                println!("You win!");
                break;
            }
        }
    }
}
