// To define a constant, rust follows all capital letters and snake case convention, these
// constant values do not change and you should explicitly define the data type and it should
// always have a value;
const MY_AGE: i32 = 32;

fn main() {
    //Note: rust uses snake case
    let name = "Foo";

    // rust supports shadowing
    let name = "Doe";

    // Note: if you create a variable in rust, it is immutable by default
    // To make it mutable, include mut in declaration
    let mut example = "Bar";

    // Note: rust has the capability to auto detect data type, but programmers also has the
    // capability to explicitly specify data types
    let population: i32 = 62_000_000;

    // Integers
    let red = 0xFA;

    // Floats
    let distance = 5.5;
    // println!("Your name is {}", name);
}
