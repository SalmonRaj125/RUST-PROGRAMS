fn pass_by_value(mut x: i32) {
    x += 10;
    println!("Inside pass_by_value: x = {}", x);
}

fn pass_by_reference(x: &mut i32) {
    *x += 10;
    println!("Inside pass_by_reference: x = {}", x);
}

fn main() {
    let mut num = 5;

    println!("Original num: {}", num);

    pass_by_value(num);
    println!("After pass_by_value: num = {}", num);

    pass_by_reference(&mut num);
    println!("After pass_by_reference: num = {}", num);
}
