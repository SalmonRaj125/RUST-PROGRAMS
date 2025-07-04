fn main() {
    let mut i = 1;

    println!("First 10 Natural Numbers:");
    loop {
        if i > 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}
