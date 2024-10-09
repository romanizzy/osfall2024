fn check_guess(guess: i32, secret: i32) -> i32{
    if guess < secret {
        return -1;
    }
    else if secret < guess {
        return 1;
    }
    else{
        return 0;
    }
}

fn main() {
    let mut secret = 70;
    let mut num_of_guesses = 0;
    let mut guess = 50;
    let mut guess_output = 0;

    loop {
        guess_output = check_guess(guess, secret);
        if guess_output == 1{
            println!("The guess: {} is too high!", guess);
            guess -= 1;
            num_of_guesses += 1;
        }
        else if guess_output == -1{
            println!("The guess: {} is too low", guess);
            guess += 1;
            num_of_guesses += 1;
        }
        else{
            println!("{} is the correct guess!", guess);
            println!("You found it in {} guesses.", num_of_guesses);
            break;
        }
    }
}
