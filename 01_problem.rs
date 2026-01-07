/*
Write a program that declares two integer variables with 
different values. Implement logic to swap their values 
so that the first variable holds the second variable's 
value and vice versa, then print the results.
*/

fn main() {
    let mut x = 34;
    let mut y = 55;
    println!("Before swaping -> x -> {} and y -> {}", x, y);
    (x,y) = (y,x);
    println!("After swaping -> x -> {} and y -> {}", x,y)
}