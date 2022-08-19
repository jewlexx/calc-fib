use std::str::FromStr;

use num_bigint::BigUint;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter a number");
        panic!();
    });

    BigUint::from_str(&arg).unwrap_or_else(|_| {
        println!("Please pass a valid number");
        panic!();
    });

    println!("Hello, world!");
}
