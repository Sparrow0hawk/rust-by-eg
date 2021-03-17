use crate::List::*;

enum List {
    // Cons: tuple struct that wraps an element and a pointer to the next node 
    Cons(u32, Box<List>),
    // Nil: a node that signifies the end of the linked list
    Nil,
}

// attach method to enum
impl List {
    // create an empty list
    fn new() -> List {
        // Nil has type List
        Nil
    }

    // consume a list and return same list with new element prepended
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because of the behaviour of the method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a 
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // base case: an empty list has zero length
            Nil => 0
        }
    }

    // return a string representation of a list
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!` but returns a heap
                // allocated string instead of printing to console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // create an empty linked list
    let mut list = List::new();

    // prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);


    // show final state of list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}