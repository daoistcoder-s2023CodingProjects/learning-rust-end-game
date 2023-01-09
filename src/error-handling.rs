#![deny(clippy::all)]

// fn get_user_name() -> Result<String, ()> {
//     Ok("John".to_string())
// }

fn get_full_name() -> Result<String, String> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn get_first_name() -> Result<String, String> {
    Err("I don't know the first name".to_string())
}

fn get_last_name() -> Result<String, String> {
    Ok("Doe".to_string())
}
fn main() {
    // returning values or errors
    // let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello World!");
    // match value {
    //     Ok(value) => println!("{}", value),
    //     Err(error) => println!("{}", error),
    // }

    // returning values or void
    // let value: Result<&str, ()> = Err(());
    // match value {
    //     Ok(value) => println!("{}", value),
    //     Err(_) => println!("Something went wrong"),
    // }

    // expecting a value from Result
    // let username = get_user_name().expect("Failed to get username");
    // println!("Hello {}", username);

    // checking ok and err with if-else statement
    // let is_ok = get_user_name().is_ok();
    // let is_err = get_user_name().is_err();
    //
    // println!("is_ok: {}", is_ok);
    // println!("is_err: {}", is_err);

    // early exit from Result Error
    let full_name = get_full_name();
    // match full_name {
    //     Ok(name) => println!("{}", name),
    //     Err(_) => println!("Error"),
    // }

    // map ok in result
    // let length = full_name.map(|str| str.len()).unwrap_or_default();
    // println!("{}", length);

    // map err in result
    let err_length = full_name.map_err(|e| e.len());
    println!("{:?}", err_length);
}
