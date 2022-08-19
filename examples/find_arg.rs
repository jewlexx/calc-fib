use std::str::FromStr;

use fibonacci_like::IntoNumber;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter a number");
        panic!();
    });

    let nth = i128::from_str(&arg).unwrap_or_else(|_| {
        println!("Please pass a valid number");
        panic!();
    });

    let number = fibonacci_like::Sequence::fibonacci().find(nth.into_number());

    println!("The \"{nth}\" number of the fibonacci sequence is:\n{number}");
}
