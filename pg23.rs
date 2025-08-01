fn calculate_area_perimeter(x: i32, y: i32) -> (i32, i32) {
    let area = x * y;
    let perimeter = 2 * (x + y);
    (area, perimeter)
}

fn main() {
    let length = 10;
    let width = 5;

    let (area, perimeter) = calculate_area_perimeter(length, width);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
