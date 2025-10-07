mod console_input;
mod subfolder;

fn main() {
    println!("Hello, world!");

    // Adding mut lets you know that this is menat to be mutable
    let mut x = 5;
    println!("x is : {}", x);
    x = 3;
    println!("x is : {}", x);

    // Constants must be defined, capitalized and immutable.
    const Y: i32 = 3;
    println!("y is {}", Y);
    const FLOATING_POINT: f32 = 10.98;
    println!("FLOATING_POINT is {}", FLOATING_POINT);

    // Tuples can contain different data types
    let tup: (bool, char, i32) = (true, 'a', 4);
    println!("bool is {}, char is {}, int is {}", tup.0, tup.1, tup.2);

    // Console input
    console_input::console_input();

    // Accessing sub modules
    subfolder::subfn::inner_fn();

    // Arrays
    let first_array: [i32; 5] = [1, 2, 3, 4, 5];

    // We use "{:?}" to print the array in a readable format else we get an error message

    println!("First array is {:?}", first_array);

    // Difference between sring and string slice in Rust

    let my_string: String = String::from("Hello"); // The "Hello" is a string slice and not a string.
    println!("my_string is {}", my_string);

    let my_string_slice: &str = "Hello"; // The "Hello" is a string slice and not a string.
    println!("my_string_slice is {}", my_string_slice);
}
