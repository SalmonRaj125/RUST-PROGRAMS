fn main() {
    println!("Multiplication Table (1 to 5):");

    for i in 1..=5 {
        for j in 1..=10 {
            print!("{:2} x {:2} = {:3}   ", i, j, i * j);
        }
        println!();
    }
}
