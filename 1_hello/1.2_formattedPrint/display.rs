use std::fmt;

// create a custom struct
#[derive(Debug)]
struct TipTop(i32, i32);

// to use {} marker  need to manually implement the fmt:display  for type

impl fmt::Display for TipTop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the first element to output stream f
        // returns a result which determines success or failure
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn main() {
    let tiptop = TipTop(10,11);

    //println!("Display: {}", tiptop);
    println!("Debug: {:?}", tiptop);
    println!("Display {}", tiptop);
}
