use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

#[cfg(test)]
mod tests {
    use crate::multi_bounds::compare_prints;

    #[test]
    fn main() {
        let string = "words";
        compare_prints(&string);

        // Error: array doesn't implement `Display`
        // let array = [1, 2, 3];
        // compare_prints(&array);
    }
}