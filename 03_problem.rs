/*
Write a program that takes three distinct integers as input.
Use a series of `if` and `else` statements to determine and
print which of the three numbers is the largest.
*/

use std::io;
use std::io::Write;

fn main() {
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let a: isize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid value");
            return;
        }
    };
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let b: isize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid value");
            return;
        }
    };
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let c: isize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid value");
            return;
        }
    };
    if a > b && a > c {
        println!("The number {} is greater than {} and {}", a, b, c)
    } else if b > a && b > c {
        println!("The number {} is greater than {} and {}", b, a, c)
    } else if c > a && c > b {
        println!("The number {} is greater than {} and {}", c, a, b)
    } else {
        println!("All numbers are equall!!!")
    }
}
