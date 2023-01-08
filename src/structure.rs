#[derive(Debug)]
struct Point(f64, f64, f64);

impl Point {
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }

    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn main() {
    let mut point = Point(1.0, 2.0, 3.0);
    let twice = point.twice();
    point.make_twice();
    println!("{:?}", point)
}
