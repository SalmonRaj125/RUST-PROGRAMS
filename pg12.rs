use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let number: i32 = input.trim().parse().expect("Please enter a valid integer");

    if number % 2 == 0 {
        println!("The number {} is Even.", number);
    } else {
        println!("The number {} is Odd.", number);
    }
}
