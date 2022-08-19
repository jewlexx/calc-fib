use std::str::FromStr;

use num_bigint::BigUint;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter a number");
        panic!();
    });

    let nth = usize::from_str(&arg).unwrap_or_else(|_| {
        println!("Please pass a valid number");
        panic!();
    });

    let number = calc_fib::calc_fib(nth);

    println!("The \"{nth}\" number of the fibonacci sequence is:\n{number}");
}
