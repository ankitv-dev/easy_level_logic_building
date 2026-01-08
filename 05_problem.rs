/*
Write a program that asks the user for a single integer.
Use a `for` loop to print the multiplication table for
that number from 1 to 10 (e.g., "5 x 1 = 5").
*/

use std::io::{self, Write};

fn main() {
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let x:i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid value");
            return;
        }
    };
    for i in 1..=10 {
        println!("{} X {} = {}", x, i, x*i);
    }
}
