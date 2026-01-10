/*
Write a program that takes an integer input (e.g., 54321).
Use a `while` loop and mathematical operations (division and modulo)
to count how many digits are in the number. Do not convert the number to a string.
*/
use std::io;
use std::io::Write;

fn main() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let mut n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid value");
            return;
        }
    };
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1
    }
    println!("{}", count)
}
