use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: i32 = input.trim().parse().expect("Please enter a valid integer");

    if number > 0 {
        println!("The number {} is Positive", number);
    } else if number < 0 {
        println!("The number {} is Negative", number);
    } else {
        println!("The number is Zero");
    }
}
