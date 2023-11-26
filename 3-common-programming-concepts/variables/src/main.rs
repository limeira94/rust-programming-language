
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60  * 3;

// fn main() {
//     let x = 5;
//     let x = x + 1;
    
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is {x}");
//     }

//     println!("The value of x is: {x}");
// }

//data types

// fn main() {
//     let sum = 5 + 10;

//     let difference = 95.5 - 14.4;

//     let product = 10 * 5;

//     let quotient = 56.7 / 23.4;

//     let remainder = 43 % 5;
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}