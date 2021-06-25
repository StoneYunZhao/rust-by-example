mod functions;
mod implementation;
mod traits;
mod bounds;
mod multi_bounds;
mod where_clauses;
mod associated_types;
mod phantom_type;

#[cfg(test)]
mod tests {
    use crate::{A, Single, SingleGen};

    #[test]
    fn main() {
        let _s = Single(A);

        let _char = SingleGen('a');
        let _t = SingleGen(A);
        let _i32 = SingleGen(6);
    }
}

struct A;

struct Single(A);

struct SingleGen<T>(T);


