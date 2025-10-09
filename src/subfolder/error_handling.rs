// enum Option<T> {
//     Some(T),
//     None,
// }
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
// fn divide_and_print(numerator: f64, denominator: f64) -> Result<f64, E> {}

pub fn error_handling() {
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("The result is: {}", value),
        None => println!("Error: Cannot divide by zero"),
    }
    // let
}
