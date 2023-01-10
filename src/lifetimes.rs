#![deny(clippy::all)]

// Note: string slice cannot be returned since there is no reference to the value being returned
// so meaning, after executing the function, the returned value is deallocated immediately -> &str

// To fix this, we need to specify to rust that the value being returned should have a lifetime
// depending on what do we specify -> &'static str
// fn get_full_name() -> &'static str {
//     "John Doe"
// }

// Generic lifetime annotation
// fn get_random_name<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
//     a
// }

// struct lifetime
// note: specify lifetime annotation in structs
// struct Person {
//     name: &'static str,
// }

// fn get_first_name(full_name: &str) -> &str {
//     full_name
// }

// Lifetime Rule #1
// Compiler assigns lifetime to every parameter that's a reference

// Lifetime Rule #2
// Single input lifetime is assigned to all outputs

// Lifetime Rule #3
// if &self or mut &self in parameters, lifetime of self is assigned to output

// struct Person<'a> {
//     first_name: &'a str,
//     last_name: &'a str,
// }

// impl<'a> Person<'a> {
//     fn first_char_of_name(&self) -> &str {
//         &self.first_name[0..1]
//     }
//
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// Lifetime in enums
enum Animal<'a> {
    Dog { name: &'a str },
}
fn main() {
    // let name = get_random_name("John", "Doe");
    // println!("{}", name);
}
