// type aliases for enums

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

// common example of using type alias with impl
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operation = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let result = Operation::Add;
    let next_result = Operation::Subtract;

    println!("result {}", result.run(10,5));
    println!("result-2 {}", next_result.run(10, 5));
}
