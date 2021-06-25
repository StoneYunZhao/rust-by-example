use std::fmt::{Display, Debug};

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);


trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

#[cfg(test)]
mod tests {
    use crate::bounds::{Rectangle, Triangle, print_debug, area};

    #[test]
    fn main() {
        // Error: Vec<T> doesn't implement `Display`
        // let s = S(vec![1]);

        let rectangle = Rectangle { length: 3.0, height: 4.0 };
        let _triangle = Triangle { length: 3.0, height: 4.0 };

        print_debug(&rectangle);
        println!("Area: {}", area(&rectangle));

        // Error: Triangle doesn't implement `Debug` or `HasArea`
        // print_debug(&_triangle);
        // println!("Area: {}", area(&_triangle));
    }
}

#[cfg(test)]
mod empty_bounds {
    struct Cardinal;

    struct BlueJay;

    struct Turkey;

    trait Red {}

    trait Blue {}

    impl Red for Cardinal {}

    impl Blue for BlueJay {}

    fn red<T: Red>(_: &T) -> &'static str { "red" }

    fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

    #[test]
    fn main() {
        let c = Cardinal;
        let b = BlueJay;
        let _t = Turkey;

        println!("A cardinal is {}", red(&c));
        println!("A blue jay is {}", blue(&b));

        // Error!
        // println!("A turkey is {}", red(&_t));
    }
}
