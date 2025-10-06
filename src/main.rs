mod console_input;
mod subfolder;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("x is : {}", x);
    x = 3;
    println!("x is : {}", x);
    const Y: i32 = 3;
    println!("y is {}", Y);
    const FLOATING_POINT: f32 = 10.98;
    println!("FLOATING_POINT is {}", FLOATING_POINT);
    let tup: (bool, char, i32) = (true, 'a', 4);
    println!("bool is {}, char is {}, int is {}", tup.0, tup.1, tup.2);

    console_input::console_input();
    subfolder::subfn::inner_fn();
}
