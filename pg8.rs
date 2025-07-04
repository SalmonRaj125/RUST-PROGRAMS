fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("Original Array: {:?}", arr);

    let slice_a = &arr[1..3];
    println!("a. Slice of 2nd and 3rd element: {:?}", slice_a);

    let slice_b = &arr[..5];
    println!("b. Slice with omitted start index (..5): {:?}", slice_b);

    let slice_c = &arr[5..];
    println!("c. Slice with omitted end index (5..): {:?}", slice_c);

    let slice_d = &arr[..];
    println!("d. Slice with both start and end omitted (..): {:?}", slice_d);
}
