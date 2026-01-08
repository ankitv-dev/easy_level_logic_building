/*
Develop a program that accepts a student's numeric score (0–100).
Classify the score into a grade (A, B, C, D, or F) based on specific
ranges (e.g., 90–100 is 'A') and print the grade.
*/

use std::io::{self, Write};

fn main() {
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let stu: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid value");
            return;
        }
    };
    println!("{}", grade(stu))
}

fn grade(marks: usize) -> String {
    if marks <= 100 && marks >= 90 {
        "Grade: A".to_string()
    } else if marks < 90 && marks >= 80 {
        "Grade: B".to_string()
    } else if marks < 80 && marks >= 70 {
        "Grade: C".to_string()
    } else if marks < 70 && marks >= 60 {
        "Grade: D".to_string()
    } else {
        "Grade: F".to_string()
    }
}
