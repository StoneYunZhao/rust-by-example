mod dead_code;
mod crates;
mod cfg;
mod custom;

#[cfg(test)]
mod tests {
    fn used_function() {}

    #[allow(dead_code)]
    fn unused_function() {}

    fn noisy_unused_function() {}

    #[test]
    fn it_works() {
        used_function();
    }
}
