use hello::greet; // more concise
use std::collections::HashMap; // another example

fn main() {
    println!("Hello, world!");
    let bunnies = 2; // immutable by default
    let mut bunnies2: i32 = 4; // mutable and with type assignment
    let(bunnies, carrots) = (8, 20);
    const A_VAR: f64 = 99.999; // all caps
    // variables are local to their scope "shadow"
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // Prints "99"
    }
    println!("{}", x); // Prints "5"
    // must initalized variables, as long as compiler can gurantee a value it works
//     hello::greet(); // bad way
    greet();
}
