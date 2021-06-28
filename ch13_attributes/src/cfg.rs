#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

mod tests {
    use crate::cfg::are_you_on_linux;

    #[test]
    fn main() {
        are_you_on_linux();

        println!("Are you sure?");
        if cfg!(target_os = "linux") {
            println!("It's linux");
        } else {
            println!("It's not linux!");
        }
    }
}