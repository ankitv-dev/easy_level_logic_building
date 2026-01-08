/*
Develop a program that accepts a student's numeric score (0–100).
Classify the score into a grade (A, B, C, D, or F) based on specific
ranges (e.g., 90–100 is 'A') and print the grade.
*/

fn main() {
    let stu: usize = 80;
    println!("{}", grade(stu))
}

fn grade(marks: usize) -> String {
    if marks >= 90 && marks <= 100 {
        "Grade: A".to_string()
    } else if marks < 90 && marks >= 80 {
        "Grade: B".to_string()
    } else if marks < 70 && marks >= 60 {
        "Grade: C".to_string()
    } else if marks < 50 && marks >= 35 {
        "Grade: D".to_string()
    } else {
        "Grade: F".to_string()
    }
}
