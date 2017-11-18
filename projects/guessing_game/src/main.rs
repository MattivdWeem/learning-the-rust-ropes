use std::io; // use the io library from the standards library

fn main() {
    // print a simple line
    println!("Guess the number!");

    // print a second line to request input
    println!("Please input your guess.");

    // create a mutable variable to capture the new string we need for the guessing
    let mut guess = String::new();

    // require the io library for stdin input, read the line for the mutable value of guess
    // if this failes, throw failed to read line error.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");


    // return your guess in a variable bound print line
    println!("You guessed: {}", guess);

}
