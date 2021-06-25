struct A;

struct S(A);

struct SGen<T>(T);

// not generic
fn reg_fn(_s: S) {}

// not generic
fn gen_spec_t(_s: SGen<A>) {}

// not generic
fn gen_spec_i32(_s: SGen<i32>) {}

// generic
fn generic<T>(_s: SGen<T>) {}

#[cfg(test)]
mod tests {
    use crate::functions::{reg_fn, A, S, gen_spec_t, SGen, gen_spec_i32, generic};

    #[test]
    fn main() {
        reg_fn(S(A));

        gen_spec_t(SGen(A));

        gen_spec_i32(SGen(6));

        generic::<char>(SGen('a'));
        generic(SGen('a'));
    }
}
