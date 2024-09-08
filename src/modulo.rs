use num_bigint::BigUint;


pub fn find_modulo(dividend: &BigUint, divisor: &BigUint) -> BigUint{
    let multiplier = dividend / divisor;
    dividend - (multiplier*divisor)
}