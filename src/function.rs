// Note: in rust it doesn't really matter the return keyword
fn say_hello_world() -> String {
    String::from("Hello World!")
}

// Note: functions with no return types return unit type
fn say_hello_world_no_return() {
    let message = String::from("Hello World");
    println!("{}", message);
}
fn main() {
    say_hello_world();
    say_hello_world_no_return();

    // Functions can be defined inline if it is not complicated
    let say_hello_world_inline = |name: &str| format!("Hello, {}!", name);
}
