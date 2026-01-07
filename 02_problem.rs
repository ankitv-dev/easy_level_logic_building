/*
Create a program that accepts a single integer input from the user. 
Use a conditional statement to check if the number is even or odd 
and print the corresponding message to the console.
*/

fn main() {
    let x:usize = 25;
    println!("{}",iseven(x));
}


fn iseven(x:usize) ->String{
    if x%2 == 0 {
        "The number is even".to_string()
    } else {
        "The number is old".to_string()
    }
}