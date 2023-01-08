#![deny(clippy::all)]

fn main() {
    // Note: Iterators in Rust is lazy

    // let values = vec![1, 2, 3, 4, 5];

    // Use for in loop to iterate over vector values
    // for value in values.iter() {
    //     println!("{}", value);
    // }

    // Note: iterators cannot be double consumed
    // let iter = values.iter();
    // let sum1: i32 = iter.sum();
    // println!("{:?}", sum1);
    // let sum2: i32 = iter.sum(); This will cause an error since the pointer would be on the
    // last element of the vector already

    // Note: Map Iterators
    // let foo = values.iter().map(|x| x * 2); -> This will give you a map iterator data type
    // println!("{:?}", foo);

    // To collect this, we need to use collecting iterator
    // let multiplied_by_2: Vec<i32> = values.iter().map(|x| x * 2).collect();
    // println!("{:?}", multiplied_by_2);

    // Unowned iterators

    // Note: by default iter goes through the reference
    let names = vec!["John", "Doe", "Mary", "Bob", "Tom"];

    // for name in names.iter() {
    //     println!("{}", name);
    // }

    // to make it goes through the value instead, use into_iter, this will own the collection
    // for name in names.into_iter() {
    //     println!("{}", name);
    // }

    // use filtering iterator
    for name in names.into_iter().filter(|name| name.len() == 3) {
        println!("{}", name);
    }
}
