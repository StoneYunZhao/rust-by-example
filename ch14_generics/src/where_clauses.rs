use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

#[cfg(test)]
mod tests {
    use crate::where_clauses::PrintInOption;

    #[test]
    fn main() {
        let vec = vec![1, 2, 3];

        vec.print_in_option();
    }
}