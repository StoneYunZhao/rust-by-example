use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

mod tests {
    use crate::phantom_type::{PhantomTuple, PhantomStruct};
    use std::marker::PhantomData;

    #[test]
    fn main() {
        let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
        let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

        let _struct1: PhantomStruct<char, f32> = PhantomStruct {
            first: 'Q',
            phantom: PhantomData,
        };

        let _struct2: PhantomStruct<char, f64> = PhantomStruct {
            first: 'Q',
            phantom: PhantomData,
        };

        // Error: type mismatch
        // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
        // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
    }
}

mod unit_clarification {
    use std::marker::PhantomData;
    use std::ops::Add;

    #[derive(Debug, Clone, Copy)]
    enum Inch {}

    #[derive(Debug, Clone, Copy)]
    enum Mm {}

    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        // add() returns a new `Length` struct containing the sum.
        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            // `+` calls the `Add` implementation for `f64`.
            Length(self.0 + rhs.0, PhantomData)
        }
    }

    #[test]
    fn main() {
        let one_foot: Length<Inch> = Length(12.0, PhantomData);
        let one_meter: Length<Mm> = Length(1000.0, PhantomData);

        let two_feet = one_foot + one_foot;
        let two_meters = one_meter + one_meter;

        // Addition works.
        println!("one foot + one_foot = {:?} in", two_feet.0);
        println!("one meter + one_meter = {:?} mm", two_meters.0);

        // Compile-time Error: type mismatch.
        // let one_feter = one_foot + one_meter;
    }
}