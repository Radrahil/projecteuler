use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};
use std::str::FromStr;

fn main() {
    // Calculate 3^1000
    let base = BigUint::from_str("2").unwrap();
    let exponent = 1000u32;
    let result = base.pow(exponent);

    // Sum the digits of the result
    let mut sum = 0;
    let mut number = result;

    while number > BigUint::zero() {
        let digit = &number % 10u32;
        sum += digit.to_u32().unwrap();
        number /= 10u32;
    }

    println!("{}", sum);
}
