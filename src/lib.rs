use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};

pub struct Sequence(pub BigInt, pub BigInt);

impl From<[i128; 2]> for Sequence {
    fn from(array: [i128; 2]) -> Sequence {
        Sequence(array[0].to_bigint().unwrap(), array[1].to_bigint().unwrap())
    }
}

pub fn calc_seq(start: impl Into<Sequence>, n: usize) -> BigInt {
    let seq: Sequence = start.into();
    let mut numbers = vec![seq.0, seq.1];

    for _ in 3..=n {
        numbers.push(numbers.get(0).unwrap() + numbers.get(1).unwrap());
        numbers.remove(0);
    }

    numbers.remove(1)
}

pub fn calc_fib(n: usize) -> BigUint {
    let mut numbers = vec![1u8.to_biguint().unwrap(), 1u8.to_biguint().unwrap()];

    // Start calculating the third because we have the first two
    for _ in 3..=n {
        numbers.push(numbers.get(0).unwrap() + numbers.get(1).unwrap());
        numbers.remove(0);
    }

    numbers.remove(1)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_bigint::BigUint;

    use super::*;

    #[test]
    fn test_get_500th() {
        let nth500 = BigUint::from_str("139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125").unwrap();

        assert_eq!(calc_fib(500), nth500);
    }
}
