pub fn control_fn(mloop: u32, mcondition: u32) {
    conditional_fn(mcondition);
    loop_fn(mloop);
}

fn conditional_fn(a: u32) {
    if a <= 0 {
        println!("Number should b egreater than zero! {}", a);
    } else if a % 2 == 0 {
        println!("Even number: {}", a);
    } else {
        println!("Odd number: {}", a);
    }
}

fn loop_fn(mut a: u32) {
    while a > 0 {
        print!("/*This is a loop: {} */", a);
        a -= 1;
    }

    // You cannot iterate over a string in rust instead you covert to a slice of char with or without indices.
    let mystring = "123456".chars();
    for item in mystring {
        println!("Char: {:?}\\n", item);
    }
}
