fn main() {
    let args = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter a number");
        panic!();
    });

    println!("Hello, world!");
}
