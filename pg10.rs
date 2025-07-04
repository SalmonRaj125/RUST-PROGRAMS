fn main() {
    let x = 10;

    let ref_x = &x;

    println!("Original value (x): {}", x);
    println!("Borrowed reference (&x): {}", ref_x);

    println!("Dereferenced value (*&x): {}", *ref_x);

    let mut y = 20;
    let ref_mut_y = &mut y;
    *ref_mut_y += 5; 
    println!("Modified value using mutable reference: {}", y);
}
