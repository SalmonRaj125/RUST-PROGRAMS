fn main() {

    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;

    println!("Integer value (i32): {}", int_val);
    println!("Converted to float (f64): {}", float_val);

    let float_num: f32 = 12.75;
    let int_num: i32 = float_num as i32;

    println!("Float value (f32): {}", float_num);
    println!("Converted to integer (i32): {}", int_num);

    let ch: char = 'A';
    let ch_code: u32 = ch as u32;
    println!("Character: {}", ch);
    println!("Unicode code point: {}", ch_code);
}
