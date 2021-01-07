// using display to handle all sequential elements of a struct
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract value using indexing
        // and create a reference
        let vec = &self.0;

        write!(f, "[")?;

        // iterate over v in vec while enumerating the iteration
        // count in `count`
        for (count, v) in vec.iter().enumerate() {
            // for every element except first, add a comma
            // use ? operator to return on errors
            if count != 0 { write!(f, ", ")?; } 
            write!(f, "{}",v)?;
        }

        // close the bracket and return a result
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2, 3, 4, 5,]);
    println!("{}", v);
}