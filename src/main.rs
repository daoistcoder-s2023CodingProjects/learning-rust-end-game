#![deny(clippy::all)]

// Note: Tuples in rust is an example of heterogeneous collections
// struct Point(f32, f32);

// fn get_values() -> (String, String, i32) {
//     ("Hello".to_string(), "World".to_string(), 30)
// }
fn main() {
    // let values = ("Hello", "world", 30);

    // Note: one way of accessing values in tuples INDIVIDUALLY
    // let hello = values.0;
    // let world = values.1;
    // let age = values.2;

    // Note: unpacking tuples
    // let (hello, world, age) = values;

    // Note: ignoring values in tuples
    // let (_, _, age) = values;

    // Note: returning tuples in a function
    // let values = get_values();
    // let (hello, _, _) = get_values();

    // Note: Vectors is a 2 dimensional data (sometimes called as array in other programming
    // languages)

    // instantiating vector with fixed size
    // let values = ["foo", "bar"];

    // Iterate over vector
    // for value in values.iter() {
    //     println!("{}", value);
    // }

    // accessing value in a specific index
    // let foo = &values[0];

    // accessing length of vector
    // let length = values.len();
    // println!("length: {}", length);

    // let numValues = [10, 20];
    // let doubled = numValues.iter().map(|x| x * 2);
    // println!("{:?}", doubled);

    // Note: this is a shorthand macro for creating a new vector
    // let mut values = vec![1, 2, 3, 4, 5];
    // let mut values2 = vec![6, 7, 8];
    // println!("Values1 = {:?}", values);
    // println!("Values2 = {:?}", values2);

    // pushing and popping values in rust
    // values.push(4);
    // let four = values.pop();

    // if you want to remove all values in the vector, use clear method
    // values.clear();
    // println!("Values are {:?}", values);

    // values.extend_from_slice(&[6, 7, 8]);
    // println!("Values are {:?}", values);

    // to move values from one vector to another, use append method
    // values.append(&mut values2);
    // println!("Values1 = {:?}", values);
    // println!("Values2 = {:?}", values2);

    // to check if a vector contains specific value
    //
    // if values.contains(&3) {
    //     println!("Values contains 3");
    // } else {
    //     println!("Values does not contain 3");
    // }

    // check if a vector is empty
    // if values.is_empty() {
    //     println!("Values is empty");
    // } else {
    //     println!("Values is not empty");
    // }
}
