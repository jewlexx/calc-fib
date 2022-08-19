#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple, lightweight library to calculate second order sequences, such as the fibonacci sequence

#[cfg(feature = "big-int")]
use num_bigint::{BigInt, ToBigInt};

#[cfg(feature = "big-int")]
type Number = BigInt;

#[cfg(not(feature = "big-int"))]
type Number = u128;

#[derive(Debug, Clone)]
pub struct Sequence(pub Number, pub Number);

impl Sequence {
    pub fn new(sequence: impl Into<Self>) -> Self {
        sequence.into()
    }

    pub fn fibonacci() -> Self {
        Self::new([1, 1])
    }

    pub fn calculate(self, n: usize) -> Number {
        let mut numbers = vec![self.0, self.1];

        for _ in 3..=n {
            numbers.push(numbers.get(0).unwrap() + numbers.get(1).unwrap());
            numbers.remove(0);
        }

        numbers.remove(1)
    }
}

impl From<[i128; 2]> for Sequence {
    fn from(array: [i128; 2]) -> Sequence {
        Sequence(array[0].to_bigint().unwrap(), array[1].to_bigint().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_get_500th() {
        let nth500 = BigInt::from_str("139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125").unwrap();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.calculate(500), nth500);
    }
}
