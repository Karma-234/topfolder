use std::io;
pub fn console_input() {
    println!("console input is live!");
    let mut my_input = String::new();

    let _ = io::stdin()
        .read_line(&mut my_input)
        .expect("failed to read line");

    println!("You entered: {}", my_input.trim());
}
