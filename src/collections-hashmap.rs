#![deny(clippy::all)]

use std::collections::HashMap;

// #[derive(Hash, Eq, PartialEq, Debug)]
// struct Person {
//     name: String,
//     age: i32,
// }
fn main() {
    // instantiate a hashmap (object in other language)
    // let mut values: HashMap<&str, &str> = HashMap::new();

    // Note: insert key value pairs into the hashmap
    // values.insert("foo", "bar");
    // println!("{:?}", values);

    // Note: check if a specific key is present in the hashmap
    // if values.contains_key("name") {
    //     println!("name exists");
    // } else {
    //     println!("name does not exist");
    // }

    // Note: To remove key, use remove method
    // values.remove("foo");
    // println!("{:?}", values);

    // Unsafely reading values in Hashmap: values["foo"]
    // Safely reading values in Hashmap

    // match values.get("foo") {
    //     Some(value) => println!("foo: {}", value),
    //     None => println!("Not found"),
    // }

    // iterate over hashmap
    // for (&k, &v) in &values {
    //     println!("{}: {}", k, v);
    // }

    // retrieve entry in hashmap
    // let entry = values.entry("foo");

    // inserting into hashmap if the key is absent
    // values.entry("name").or_insert("John Doe");
    // println!("{:?}", values);

    // instantiate hashmap from struct
    // let mut person_values: HashMap<&Person, &str> = HashMap::new();
}
