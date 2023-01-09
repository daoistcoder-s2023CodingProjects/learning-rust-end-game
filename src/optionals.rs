#![deny(clippy::all)]

fn main() {
    // instantiate options
    // let value = Some(10);
    // let name: Option<&str> = Some("John Doe");

    // unwrapping options safely
    // match name {
    //     Some(name) => {
    //         println!("name: {}", name);
    //     }
    //     None => {
    //         println!("no name");
    //     }
    // }

    // unwrapping options unsafely
    // let unwrapped_name = name.expect("name is not expected");
    // println!("name: {}", unwrapped_name);

    // force unwrapping options -> usually not a good idea
    // let unwrapped_name = name.unwrap();
    // println!("name: {}", unwrapped_name);

    // mutate option value
    // let mut age: Option<i8> = Some(20);
    // match age.as_mut() {
    //     Some(age) => *age += 10,
    //     None => {}
    // }
    // println!("Age is {}", age.unwrap());

    // unwrapping multiple optionals with tuples
    // let mut age1: Option<i8> = Some(20);
    // let mut age2: Option<i8> = Some(30);
    // let mut age3: Option<i8> = Some(40);
    //
    // if let (Some(age1), Some(age2), Some(age3)) = (age1, age2, age3) {
    //     println!("{}", age1 + age2 + age3);
    //     println!("{}, {}, {}", age1, age2, age3);
    // }

    // unwrapping with default value
    // let name: Option<&str> = None;
    // let unwrapped = name.unwrap_or("World");
    // println!("name: {}", unwrapped);

    // check if option is some or none
    // if name.is_some() {
    //     println!("There is a value");
    // } else {
    //     println!("There is no value");
    // }

    // mapping an option
    // let age: Option<i32> = Some(20);
    // let age_multiply_by_2 = age.map(|age| age * 2);
    // println!("age: {}", age_multiply_by_2.unwrap_or_default());
}
