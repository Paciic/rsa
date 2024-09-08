use crate::modulo;
use num_bigint::{BigUint, ToBigUint};
/*
fn pow(n: BigUint, exp: BigUint) -> BigUint {
    n.pow(exp.try_into().expect("exponent too large for pow()"))
}
*/
pub fn crypt(messages: Vec<u8>, encryptor: u128, modulo: u64) -> Vec<BigUint> {

    let mut k: Vec<BigUint> = Vec::new();
    for message in messages {
        //CONV to u32768
        let big_message: BigUint = message.to_biguint().unwrap();
        let big_mod = modulo.to_biguint().unwrap();
        let mut big_c =  1.to_biguint().unwrap();

        //perform modulo function
        for _exponent in 0..encryptor{
            big_c = &big_message * &big_c;
            big_c = modulo::find_modulo(&big_c, &big_mod);
        }

        //add to end
        k.push(big_c.clone());
    }
    //return
    k
}