use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number");
    //Generates random number between 1 and 100 to have the user guess against
    let secret_number = rand::thread_rng()
        .gen_range(1,101);

    //User guess loop
    loop {
        println!("Input your guess");

        //Initializes the user guess variable as a string
        let mut guess = String::new();

        //Reads user input and stores the result in the guess variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //Parses the guess variable into a number through shadowing, restarting the loop if this
        // process fails
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        //Prints the user's guess
        println!("You guessed: {}", guess);

        //Compares the user's guess against the previously generated secret_number, and
        //tells the user if they were above or below the secret_number. We break out of the loop if
        //the user guesses correctly.
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
