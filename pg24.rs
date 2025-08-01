fn arr_square(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|x| x * x).collect()
}

fn main() {
    let input = [1, 2, 3, 4, 5];
    let squared = arr_square(&input);

    println!("Original: {:?}", input);
    println!("Squared: {:?}", squared);
}
