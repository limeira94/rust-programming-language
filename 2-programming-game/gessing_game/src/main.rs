use std::io;

fn main() {
    println!("Guess the number!");

    println!("PLease input your guess.");

    let mut guess = String::new();  // let apples =5; immutable
                                    // let mut apples = 5; // mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
