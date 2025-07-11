fn main() {
    for i in 1..=5 {
        let line = std::iter::repeat(i.to_string())
            .take(i)
            .collect::<String>();
        println!("{}", line);
    }
}
