use num_bigint::{BigUint, ToBigUint};

pub fn pow(base: BigUint, exponent: BigUint) -> BigUint {
    let mut result:BigUint = Default::default();
    let mut i = 0.to_biguint().unwrap();
    while i < exponent{
        result = result * &base;
        i = i + 1.to_biguint().unwrap();
    }
    return result;
}


