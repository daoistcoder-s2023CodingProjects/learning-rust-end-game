fn main() {
    // Note: In rust if you move value, it is automatically deallocated in the first variable
    // after moving.
    let name = String::from("John");
    let name2 = name;

    // NOTE: if the variable is move out of scope, then that variable is drop automatically
    {
        let age = 32;
        println!("{}", age);
    }

    // println!("{}", age); -> Here you are going to have some errors, telling you that age is
    // not defined
}
