use std::fmt;
// tuples can be used as function arguments and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let can be used to bind members of tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {

    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// the following struct is for activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}),\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // a tuple with lots of types

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                    'a', true);
    
    // values can be extracted from tuple with indexing
    println!("Long tuple first value is {}", long_tuple.0);
    // cannot access last element in tuple like below
    //println!("Long tuple last value is {}", long_tuple.last());

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // but long tuples cannot be printed
    //let too_big_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    //println!("show me the tuple: {:?}", too_big_tuple);
    // this will fail

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // create a one element tuple by adding comma and surronding with parentheses
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // tuples can be deconstructed to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    println!("Regular matrix:\n{}", matrix);

    println!("Tranposed matrix:\n{}", transpose(matrix));
}