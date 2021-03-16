// an attribute to hide warnings for unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly `use` each name so they are available without
    // manual scoping

    use crate::Status::{Poor,Rich};
    // automatically use each name inside Work
    use crate::Work::*;

    // equivalent to `Status::Poor`
    let status = Poor;
    // equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        // note lack of scoping because of explicit use of `use` above!
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have not enough money..."),
    }

    match work {

        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight"),
    }
}