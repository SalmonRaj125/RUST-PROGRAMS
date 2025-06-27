fn main() {
    let x = 5;
    {
        let x = x + 1;
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);
}
