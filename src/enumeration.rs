#![deny(clippy::all)]
// Note: 1st example of enum
// #[derive(Debug, PartialEq)]
// enum AnimalType {
//     Dog,
//     Cat,
//     Rabbit,
// }

// Note: 2nd example of enum with associate values
// enum Shapes {
//     Circle { radius: f64, center: (f64, f64) },
//     Rectangle { width: f64, height: f64 },
// }

// struct Size {
//     width: f32,
//     height: f32,
// }
//
// enum Shapes {
//     Rectangle(f32, f32, Size),
//     Circle(f32, f32, f32),
// }
//
// impl Shapes {
//     fn area(&self) -> f32 {
//         match self {
//             Shapes::Rectangle(_x, _y, size) => size.width * size.height,
//             Shapes::Circle(_x, _y, radius) => 3.14 * radius * radius,
//         }
//     }
// }

enum Pet {
    Cat { name: String },
    Dog { name: String },
}

fn main() {
    // Note: 1st instantiation of 1st enum
    // let fluffy = AnimalType::Cat;
    //
    // match fluffy {
    //     AnimalType::Dog => println!("It's a Dog"),
    //     AnimalType::Cat => println!("It's a Cat"),
    //     AnimalType::Rabbit => println!("It's a Rabbit"),
    // }

    // Note: 2nd instantiation of 2nd enum
    // let rectangle = Shapes::Rectangle {
    //     width: 3.0,
    //     height: 4.0,
    // };

    // Note: This is an example of if condition checking
    // if let Shapes::Rectangle { width, height } = rectangle {
    //     println!("width = {}, height = {}", width, height);
    // }

    // Note: This is an example of switch case in rust
    // match rectangle {
    //     Shapes::Rectangle { width, height } => {
    //         println!("width = {}, height = {}", width, height);
    //     }
    //     _ => println!("Not a rectangle"),
    // }

    // let rectangle = Shapes::Rectangle(
    //     1.0,
    //     2.0,
    //     Size {
    //         width: 3.0,
    //         height: 4.0,
    //     },
    // );

    // if let Shapes::Rectangle(x, y, Size { width, height }) = rectangle {
    //     println!(
    //         "x = {}, y = {}, width = {}, height = {}",
    //         x, y, width, height
    //     );
    // }

    // match rectangle {
    //     Shapes::Rectangle(x, y, Size { width, height }) => {
    //         println!(
    //             "x = {}, y = {}, width = {}, height = {}",
    //             x, y, width, height
    //         );
    //     }
    //     _ => println!("Not a rectangle"),
    // }
    //
    // let area = rectangle.area();
    // println!("Area = {}", area);

    let fluffy = Pet::Cat {
        name: "Fluffy".to_string(),
    };

    // Note: match as an expression
    let name = match fluffy {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };

    println!("Hello {}", name);
}
