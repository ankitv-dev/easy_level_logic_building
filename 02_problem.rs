/*
Create a program that accepts a single integer input from the user.
Use a conditional statement to check if the number is even or odd
and print the corresponding message to the console.
*/

use std::io;
use std::io::Write;

fn main() {
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");

    let x: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid input");
            return;
        }
    };
    println!("{}", iseven(x));
}

fn iseven(x: usize) -> String {
    if x % 2 == 0 {
        "The number is even".to_string()
    } else {
        "The number is old".to_string()
    }
}
