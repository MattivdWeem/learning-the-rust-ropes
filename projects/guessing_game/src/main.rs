extern crate rand;// use the external rand lib
use std::io; // use the io library from the standards library
use std::cmp::Ordering; // from standards, get compare
use rand::Rng; // from the rand lib, use the Range lib

fn main() {
    // print a simple line
    println!("Guess the number!");

    // create the random number between 1 and 101 using the rand range lib
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // print the secret number
    println!("The secret number is: {}", secret_number);

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

    // ccheck if the input is a string or an number
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    // match one of the multiple options with the value, use the compar library
    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }





}
