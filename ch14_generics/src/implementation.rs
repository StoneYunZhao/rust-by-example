struct S;

struct GenericVal<T>(T);

// implement by explicitly specify type parameter
impl GenericVal<f32> {}

impl GenericVal<S> {}

// implement remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}


#[cfg(test)]
mod tests {
    use crate::implementation::{Val, GenVal};

    #[test]
    fn main() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value())
    }
}