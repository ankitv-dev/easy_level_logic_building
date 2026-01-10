/*
Write a program that accepts an integer (e.g., 1234). 
Using a loop and mathematical operations (multiplication, division, and modulo) 
construct a new integer that is the reverse of the input (e.g., 4321) and print it.
*/

use std::io::{self, Write};

fn main() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let mut n:usize = match input.trim().parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("Please enter valid value");
            return;
        }
    };
    let mut reverse = 0;
    while n > 0{
        let rem = n%10;
        reverse = reverse*10+rem;
        n /=10
    }
    println!("{}", reverse)
}