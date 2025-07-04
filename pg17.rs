fn main() {
    let mut count = 0;
    let mut value = 1;

    while value < 100 {
        value *= 2; 
        count += 1;
        println!("Iteration {}: Value = {}", count, value);
    }

    println!("Total iterations until value >= 100: {}", count);
}
