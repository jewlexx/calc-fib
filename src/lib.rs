#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

/// An error for when the given input could not be found in the sequence
#[derive(Debug)]
pub enum FindError {
    /// The input was not found in the sequence
    NotFound(Number),
}

impl core::fmt::Display for FindError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FindError::NotFound(number) => {
                write!(f, "The number `{}` was not found in the sequence", number)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FindError {}

type FindResult<T> = Result<T, FindError>;

/// The into_number trait
///
/// See [`Number`] for more information.
pub trait IntoNumber {
    /// Converts the given value into a [`Number`] type
    fn into_number(self) -> Number;
}

/// The number return type
///
/// Will be either [`num_bigint::BigInt`] or [`i128`] based on whether the `big-int` feature is enabled or not
#[cfg(feature = "big-int")]
pub type Number = num_bigint::BigInt;

/// The number return type
///
/// Will be either [`num_bigint::BigInt`] or [`i128`] based on whether the `big-int` feature is enabled or not
#[cfg(not(feature = "big-int"))]
pub type Number = i128;

impl IntoNumber for i128 {
    fn into_number(self) -> Number {
        cfg_if::cfg_if! {
            if #[cfg(feature = "big-int")] {
                use num_bigint::ToBigInt;

                self.to_bigint().unwrap()
            } else {
                self
            }
        }
    }
}

fn update_array(numbers: &mut [Number; 2]) {
    let old_x = &numbers[0];
    let old_y = &numbers[1];
    let new_y = old_x + old_y;
    // Removes the need to clone if we add 0
    numbers[0] = old_y + 0;
    numbers[1] = new_y;
}

/// A sequence, represented by the two starting values, which are later used to compute further values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence(pub Number, pub Number);

impl Sequence {
    /// Creates a new sequence with the given starting values
    ///
    /// # Examples
    ///
    /// ```
    /// # use fibonacci_like::Sequence;
    /// Sequence::new([1,1]);
    /// ```
    pub fn new(sequence: impl Into<Self>) -> Self {
        sequence.into()
    }

    /// Returns the fibonacci sequence, represented as a [`Sequence`] struct
    ///
    /// # Examples
    ///
    /// ```
    /// # use fibonacci_like::Sequence;
    /// let sequence = Sequence::new([1, 1]);
    /// let fib_sequence = Sequence::fibonacci();
    ///
    /// assert_eq!(sequence, fib_sequence);
    /// ```
    pub fn fibonacci() -> Self {
        Self::new([1, 1])
    }

    /// Calculate the nth term of the sequence
    ///
    /// # Examples
    ///
    /// ```
    /// # use fibonacci_like::{Sequence, IntoNumber};
    /// let sequence = Sequence::fibonacci();
    /// let nth_term = sequence.calculate(3);
    ///
    /// assert_eq!(nth_term, 2_i128.into_number());
    /// ```
    pub fn calculate(self, n: usize) -> Number {
        let mut numbers = [self.0, self.1];

        for _ in 2..n {
            update_array(&mut numbers);
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "big-int")] {
                use num_bigint::ToBigInt;
                numbers[1].to_bigint().unwrap()
            } else {
                numbers[1]
            }
        }
    }

    /// Find the given number's position in the sequence
    ///
    /// # Examples
    ///
    /// ```
    /// # use fibonacci_like::{Sequence, IntoNumber};
    /// let fifteenth = 610.into_number();
    ///
    /// let fib = Sequence::fibonacci();
    ///
    /// assert_eq!(fib.find(fifteenth).unwrap(), 15);
    /// ```
    pub fn find(self, number: Number) -> FindResult<usize> {
        let mut numbers = [self.0, self.1];

        if number == numbers[0] {
            return Ok(1);
        } else if number == numbers[1] {
            return Ok(2);
        }

        let mut n = 2;
        loop {
            update_array(&mut numbers);
            n += 1;

            if numbers[1] > number {
                break Err(FindError::NotFound(number));
            }

            if numbers[1] == number {
                break Ok(n);
            }
        }
    }
}

impl From<[i128; 2]> for Sequence {
    fn from(array: [i128; 2]) -> Sequence {
        Sequence(array[0].into_number(), array[1].into_number())
    }
}

#[cfg(test)]
mod tests {
    cfg_if::cfg_if! {
        if #[cfg(feature = "big-int")] {
            use num_bigint::BigInt;
            use std::str::FromStr;
        }
    }

    use super::*;

    // This test does not work without big-int as the literal is too large to fit in i128
    #[cfg(feature = "big-int")]
    #[test]
    fn test_get_500th() {
        let nth500 = BigInt::from_str("139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125").unwrap();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.calculate(500), nth500);
    }

    #[test]
    fn test_get_first() {
        let first = 1.into_number();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.calculate(1), first);
    }

    #[test]
    fn test_get_third() {
        let third = 2.into_number();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.calculate(3), third);
    }

    #[test]
    fn test_find_first() {
        let first = 1.into_number();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.find(first).unwrap(), 1);
    }

    #[test]
    fn test_find_third() {
        let third = 2.into_number();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.find(third).unwrap(), 3);
    }

    #[test]
    fn test_find_fifteenth() {
        let fifteenth = 610.into_number();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.find(fifteenth).unwrap(), 15);
    }
}
