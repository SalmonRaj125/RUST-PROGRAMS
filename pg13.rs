use std::io;

fn main() {
    println!("Enter first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read input");
    let operator = operator.trim();  // Remove newline

    println!("Enter second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Using match expression for operations
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero is not allowed.");
                return;
            }
        },
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
