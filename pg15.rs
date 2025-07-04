fn main() {
    let mut i = 1;

    println!("First 10 Natural Numbers:");

    // Using an infinite loop with break condition
    loop {
        if i > 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}
