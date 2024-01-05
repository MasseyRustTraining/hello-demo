fn hello_string(location: &str) -> String {
    location.to_string()
}

/// Print "hello world" as per the standard thing.
fn main() {
    let location = "world";
    let mut nlocation = location.len();
    nlocation -= 1;
    for i in (0..nlocation).rev() {
        println!("{}: hello {}", i, hello_string(location));
    }
}
