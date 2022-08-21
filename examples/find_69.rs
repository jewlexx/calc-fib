fn main() {
    let nth = 69;

    let number = fibonacci_like::Sequence::fibonacci().calculate(nth);

    println!("The \"{nth}\" number of the fibonacci sequence is:\n{number}");
}
