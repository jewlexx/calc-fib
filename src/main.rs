use std::str::FromStr;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter a number");
        panic!();
    });

    let nth = usize::from_str(&arg).unwrap_or_else(|_| {
        println!("Please pass a valid number");
        panic!();
    });

    let number = fibonacci_like::calc_fib(nth);

    println!("The \"{nth}\" number of the fibonacci sequence is:\n{number}");
}
