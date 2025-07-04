use std::io;

fn main() {
    // Ask user to enter a number
    println!("Enter a number:");

    // Read input from user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Convert input string to a number (i32)
    let number: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Check if number is positive, negative or zero
    if number > 0 {
        println!("The number {} is Positive", number);
    } else if number < 0 {
        println!("The number {} is Negative", number);
    } else {
        println!("The number is Zero");
    }
}
