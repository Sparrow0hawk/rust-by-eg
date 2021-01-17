// an array is a collection of objects with the same type, stored in contiguous memory
// created using [], there length is part of their type signature

// slices are similar to arrays but length isn't known at compile time
// a slice is a two word object, first is a pointer to data, second is the length
// word size is same as usize, slices can be used to borrow a section of an array

use std::mem;

// function to borrow slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice is: {}", slice[0]);
    println!("The slice is {} so many items long.", slice.len());
}

fn main() {
    // fixed length array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // initiate all elements with the same value
    let ys: [i32; 500] = [0; 500];

    // do some indexing
    println!("First element of xs is: {}", xs[0]);
    println!("Second element of ys is: {}", ys[0]);

    println!("Number of elements in xs: {}", xs.len());

    println!("The ys occupies {} bytes.", mem::size_of_val(&ys));

    println!("Borrowing arrays as slices");
    analyze_slice(&ys);

    // slices can point to section of array
    // [starting_point..ending_index]
    // starting_point is the first position
    // ending_index is +1 the last position
    println!("Borrow a section of an array");
    analyze_slice(&ys[1..4]);

    //println!("Out of index slice fails: {}",xs[5]);
}