#![deny(clippy::all)]

use intutils::addition::add;
use intutils::subtraction::subtract;
fn main() {
    let added = add(1, 2);
    let subtracted = subtract(1, 2);

    println!("{}", added);
    println!("{}", subtracted);
}
