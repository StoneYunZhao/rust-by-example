#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met");
}


// rustc --cfg some_condition custom.rs