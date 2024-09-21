use num_bigint::BigUint;


pub fn find_bigint_modulo(dividend: &BigUint, divisor: &BigUint) -> BigUint{
    let multiplier = dividend / divisor;
    dividend - (multiplier*divisor)
}
pub fn find_u128i32_modulo(dividend: &u128, i32_divisor: &i32) -> u128{
    let divisor = u128::from(i32_divisor);
    let multiplier = dividend / divisor;
    dividend - (multiplier*divisor)
}