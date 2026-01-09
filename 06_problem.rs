/*
Create a program that accepts a positive integer `N`. 
Use a loop to calculate the sum of all numbers from 1 up to 
and including `N`, then display the total.
*/
use std::io::{self, Write};


fn main() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let n:usize = match input.trim().parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("Please enter vaild value");
            return;
        }
    };
    let mut t:usize = 0;
    for i in 1..=n{
        t+=i;
    }
    println!("{}",t)
}