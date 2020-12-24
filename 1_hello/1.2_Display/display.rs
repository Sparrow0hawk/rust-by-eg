use std::fmt;

// create a custom struct
#[derive(Debug)]
struct TipTop(i32, i32);

fn main() {
    let tiptop = TipTop(10,11);

    //println!("Display: {}", tiptop);
    println!("Debug: {:?}", tiptop);
}
