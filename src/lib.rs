use num_bigint::{BigUint, ToBigUint};

pub fn calc_fib(n: usize) -> BigUint {
    let mut numbers = vec![1u8.to_biguint().unwrap(), 1u8.to_biguint().unwrap()];

    for _ in 0..=n {
        numbers.push(numbers.get(0).unwrap() + numbers.get(1).unwrap());
        numbers.remove(0);
    }

    numbers.remove(1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_500th() {
        // let nth500 = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125u128;
    }
}
