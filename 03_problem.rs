/*
Write a program that takes three distinct integers as input. 
Use a series of `if` and `else` statements to determine and 
print which of the three numbers is the largest.
*/

fn main() {
    let a = 45;
    let b = 45;
    let c = 45;
    if a > b && a > c{
        println!("The number is {} is greater than {} and {}", a, b ,c)
    } else if b > a && b > c{
        println!("The number is {} is greater than {} and {}", b, a, c)
    } else if c > a && c > b{
        println!("The number is {} greater than {} and {}", c, a, b)
    } else{
        println!("All numbers are equall!!!")
    }
}