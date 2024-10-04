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
    let mut secret = 68;
    let mut num_of_guesses = 0;
    let mut guess = 50;
    let mut right_guess = false;
    let mut guess_output = 0;

    while right_guess == false{
        guess_output = check_guess(guess, secret);
        if guess_output == 1{
            println!("The guess: {} is too high!", guess);
            guess -= 1;
            num_of_guesses += 1;
        }
        else if guess_output == 0{
            println!("{} is the correct guess!", guess);
            println!("You found it in {} guesses.", num_of_guesses);
            right_guess = true;
        }
        else{
            println!("The guess: {} is too low", guess);
            guess += 1;
            num_of_guesses += 1;
        }
    }
}
