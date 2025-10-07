// Ownership, references, and borrowing

pub fn orb_fn() {
    // In reference, you can have multiple immutable references to the same mutable data
    let mut x: u32 = 11;
    println!("x is: {}", x);
    x += 1; // This is possible because x has not been referenced yet.
    let y = &x; // y is a reference to x
    println!("x is: {}", x); // Prints 12
    // x += 1; *This will throw an error because x is borrowed by y.
    println!("y is: {}", y);

    let mut j = &y; // j is a mutable reference to y
    println!("j is: {}", j); // Prints 12
    // *j += 1; // This is impossible because j is a mutable reference to immutable y.
    println!("j is: {}", j); // Prints

    let z = &mut x; // z is a mutable reference to x
    println!("z is: {}", z); // Prints 12
    *z += 1; // This is possible because z is a mutable reference to x.

    println!("z is now {} ", z);

    // In borrowing, you can have multiple mutable references to the same data, but not immutable references
}
