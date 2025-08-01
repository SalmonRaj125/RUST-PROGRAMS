fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 7;
    let result = fibonacci(n);
    println!("Fibonacci term at position {} is {}", n, result);
}
