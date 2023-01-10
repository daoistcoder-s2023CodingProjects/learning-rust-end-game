#![deny(clippy::all)]

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

// trait HasFullName {
//     fn full_name(&self) -> String;
// }

// Note: trait in another trait

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

trait HasFullName
where
    Self: HasName,
{
    fn full_name(&self) -> String;
}

impl<T> HasFullName for T
where
    T: HasName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

// Note: Trait as parameters
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

// single trait in trait bound syntax
// fn print_details<T: HasFullName>(value: &T) {
//     println!("{}", value.full_name());
// }

// multiple trait in trait bound syntax
// fn print_details<T: HasFullName + CanRun>(value: &T) {
//     println!("{}", value.full_name());
//     value.run();
// }

// Trailing bounds using where keyword
fn print_details<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name());
    value.run();
}

trait CanRun {
    fn run(&self);
}

// impl HasFullName for Person {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

impl CanRun for Person {
    fn run(&self) {
        todo!()
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(" ").collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 30,
        }
    }
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }
}

// Note: Trait bound syntax

fn main() {
    let person = Person::new("John Doe");
    // print_full_name_and_age(&person);
    println!("{}", person.last_name());
}
