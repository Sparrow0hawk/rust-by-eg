// type aliases for enums

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

// create a type alias
type Operation = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // can refer to each variant via the alias
    let x = Operation::Add;
}

