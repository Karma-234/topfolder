pub fn function_types() {
    // Function with no return type
    let no_return_fn: fn() = || println!("No return");
    no_return_fn();

    // Function with return type
    let return_fn: fn() -> u32 = || 42;
    println!("Return value: {}", return_fn());

    // Function with multiple arguments and return type
    let multi_fn: fn(u32, u32) -> u32 = |a, b| a + b;
    println!("Addition: {}", multi_fn(5, 10));

    // Function as a statement.`i.e it does not return a value`
    let x = statement_fn();
    println!("x is: {:?}", x);

    // Expression function`
    let expr_result = expression_fn();
    println!("Expression result: {}", expr_result);

    // Expression function with semicolon
    let expr_result2 = expression_fn2();
    println!("Expression result2: {}", expr_result2);
}

fn statement_fn() {
    // This will return nothing
    let my_num: i32 = 1 + 2;
    println!("This is a statement function {}", my_num);
}

// This is an example of an expression function
fn expression_fn() -> i32 {
    // This will return the last line of the code body by ommitting the semicolon
    let a = 1;
    let b = 2;
    a + b
}

fn expression_fn2() -> i32 {
    // This will return the last line of the code body with the semicolon and return statement.
    let a = 1;
    let b = 2;
    return a + b;
}
