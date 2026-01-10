/*
Write a function that calculates the factorial of a non-negative integer provided by the user.
The logic should use a loop to multiply the numbers and return the result for printing.
*/

use std::io::{self, Write};

fn main() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid value");
            return;
        }
    };
    let mut t: usize = 1;
    for i in 2..=n {
        t *= i
    }
    println!("{}", t)
}
