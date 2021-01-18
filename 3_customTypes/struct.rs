// 3 types of structs can be created with struct keyword
//  a tuple struct
// a classic C struct
// a unit struct, fieldless

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple pair
struct Pair(i32, f32);

// a struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// structs can also be reused as fields for another struct
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    // instaniated point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // make a new point using struct update syntax to use the fields of another point
    let bottom_right = Point {x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point using let
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);
}