fn test_divisibility_by_3_4(n: i32) -> i32 {
    if n % 3 == 0 && n % 4 == 0 {
        0
    } else if n % 3 == 0 {
        1
    } else if n % 4 == 0 {
        2
    } else {
        -1
    }
}

fn main() {
    let number = 12;

    let result = test_divisibility_by_3_4(number);
    println!("Result for {}: {}", number, result);
}
