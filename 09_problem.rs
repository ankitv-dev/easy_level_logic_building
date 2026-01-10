/*
Create a program that accepts a positive integer.
Implement logic using a loop to check if the number is divisible
by any number other than 1 and itself.
Print "Prime" or "Not Prime" based on the result.
*/
use std::io;
use std::io::Write;

fn main() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");
    if n <= 1 {
        println!("Not Prime");
        return;
    }
    let mut is_prime = true;
    let mut i = 2;
    while i <= n / 2 {
        if n % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }
    if is_prime {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}
