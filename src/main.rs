mod console_input;
mod subfolder;

fn main() {
    println!("Hello, world!");

    // Varibales are orignally immutable in rust. Adding mut lets you know that this is meant to be mutable.
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

    // * Differences between sring and string slice in Rust. *

    // String` is a growable, mutable string type.` i.e you can add, reduce the characters in it.
    // String or declared variables are usually stored on the heap (they are ususally growable) allowing efficient runtime memory management.
    // Varriables decalerd on the stack (they are usually not growable) are accessed faster which can lead to more efficient memory usage.
    let mut my_string: String = String::from("Hello"); // The "Hello" is a string slice and not a string.
    println!("my_string is {}", my_string);
    my_string.push_str("string");
    println!("my_string is now {}", my_string);

    // mut should also be throwing a warning here because the slice is immutable.
    let mut my_string_slice: &str = "Hello"; // The "Hello" is a string slice and not a string.
    println!("my_string_slice is {}", my_string_slice);
    my_string.push_str("new slice");
    println!("my_string_slice is is now {}", my_string_slice); // This will print "Hello". The string slice is not mutable.

    // Function types
    subfolder::functions::function_types();

    // Ownership, references, and borrowing
    subfolder::orb::orb_fn();

    // Control flow
    subfolder::control_flow::control_fn(3, 5);

    // Structs
    subfolder::structs::my_struct();

    // Error handling
    subfolder::error_handling::error_handling();

    // Modules
    println!("Modules are working!");
}
