fn main() {
    let some_number = Some(10);

    if let Some(value) = some_number {
        println!("Matched! The value is: {}", value);
    } else {
        println!("No value found (None).");
    }

    let no_number: Option<i32> = None;

    if let Some(value) = no_number {
        println!("Matched! The value is: {}", value);
    } else {
        println!("No value found (None).");
    }
}
