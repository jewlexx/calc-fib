pub fn calc_fib(n: usize) -> u128 {
    let mut numbers = [1u128, 1u128];

    for _ in 0..=n {
        let old_x = numbers[0];
        let old_y = numbers[1];
        numbers[0] = old_y;
        numbers[1] = old_x + old_y;
    }

    numbers[1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_500th() {
        let nth500 = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125u128;
    }
}
