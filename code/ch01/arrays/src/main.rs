fn main() {
    // Arrays have a fixed size.
    // All elements are of the same type.
    let array = [1, 2, 3, 4, 5];

    // Create an array of 20 elements where all elements are the same.
    // The size should be a compile-time constant.
    let ones = [1; 20];

    // Get the length of an array.
    println!("Length of ones: {}", ones.len());

    // Access an element of an array.
    // Indexing starts at 0.
    println!("Second element of array: {}", array[1]);

    if array.len() >= 6 {
        // Run-time bounds-check.
        // This panics with 'index out of bounds: the len is 5 but the index is 5'.

        let value2 = 5;
        let value3 = ||  value2;
        println!("Non existant element of array: {}", array[value3()]);
    }
}
