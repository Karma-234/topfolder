use std::collections::HashMap;

pub fn vuh() {
    // There are two ways to use vectors in Rust:

    let my_vec: Vec<i32> = Vec::new();
    // This creates an empty vector without any elements.
    let mut my_svec = vec![1, 2, 3, 4, 5];
    // This creates a mutable vector with some elements.

    // Adding elements to a vector
    my_svec.push(6);
    // Removing elements from a vector
    my_svec.pop();

    // Iterating over a vector
    for num in &my_vec {
        println!("{}", num);
    }

    // Accessing vectors by index
    let first_element = &my_svec[0];

    // Hashmaps are a collection of key-value pairs.
    let mut my_hash = HashMap::new();

    //inserting key-value pairs into a hashmap
    my_hash.insert(1, "one");

    // Removing key-value pairs from a hashmap
    my_hash.remove(&1);
}
