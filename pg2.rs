fn main() {
    for i in 1..=5 {
        // Repeat the character i times and print it
        let line = std::iter::repeat(i.to_string())
            .take(i)
            .collect::<String>();
        println!("{}", line);
    }
}
