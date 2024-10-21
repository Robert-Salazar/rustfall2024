fn main() {
    let secret_num = 22; // Hard-coded key to guessing game
    let mut counter = 0; // Tracks attempts
    let mut guess = 12; // Simulate starting guess

    loop {
        counter += 1; // Increment guess attempt counter

        let result = check_guess(guess, secret_num); // Call check_guess to compare

        if result == 0 {
            println!("Correct! The secret number is {}.", guess);
            break;
        } else if result == 1 {
            println!("The guess is too high");
            guess -= 1; // Decrease the guess
        } else {
            println!("The guess is too low");
            guess += 1; // Increase the guess
        }
    }

    println!("It took {} guesses to find the correct number.", counter);
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
       -1
    }
}
