#![deny(clippy::all)]

// use std::rc::Rc;
use std::cell::Cell;

struct Person {
    name: String,
    age: Cell<u8>,
}

impl Person {
    fn increment_age(&self) -> u8 {
        self.age.set(self.age.get() + 1);
        self.age.get()
    }
}
// use std::ops::Deref;
//
// struct BoxedValue<T> {
//     value: T,
// }
//
// impl<T> BoxedValue<T> {
//     fn new(value: T) -> Self {
//         BoxedValue { value }
//     }
// }
//
// impl<T> Deref for BoxedValue<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

// Note: Stack is LIFO (Last in First Out) while Heap is random access
fn main() {
    // Note: Box data type = stored values in the heap

    // let age = Box::new(22);
    // dereferencing with box data type
    // let twice = *age * 2;
    // println!("{}", age);

    // let age = BoxedValue::new(22);
    // println!("Value is {}", age); // here you cannot dereference the age value since we do not
    // have implementation of Deref Trait yet in our BoxedValue
    // println!("Value is {}", *age);

    // dereference value manually
    // let actual_age = age.deref();
    // println!("{}", actual_age);

    // point to the underlying reference
    // let actual_age = *age;

    // the *age is a shorthand for *(age.deref());

    // let value = BoxedValue::new(10);
    // print_integer(&value);

    // Note: Rc stands for reference counted
    // Rc is single threaded
    // weak reference won't own the data

    // let array = vec!["John".to_string(), "Jane".to_string()];
    // let rc = Rc::new(array);
    // let weak = Rc::downgrade(&rc);
    // drop(rc);
    // println!("{:?}", weak.upgrade().unwrap())
    // Note: Weak references won't hold onto the underlying data

    // match weak.upgrade() {
    //     Some(rc) => println!("{:?}", rc),
    //     None => println!("None"),
    // }

    // Note: cloning an rc creates a new RC object
    // let rc2 = rc.clone();
    // drop(rc);
    // println!("{:?}", rc2);

    // mutability or rc
    // rc doesn't allow this but std::cell::Cell does

    let person = Person {
        name: "John".to_string(),
        age: Cell::new(20),
    };

    let new_age = person.increment_age();
    println!("{}", new_age);
}

// Implicit deref coersion in functions
// fn print_integer(value: &i32) {
//     println!("{}", value);
// }
