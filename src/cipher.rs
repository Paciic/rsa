use num_bigint::BigUint;
use num_bigint::ToBigUint;

/*
fn pow(n: BigUint, exp: BigUint) -> BigUint {
    n.pow(exp.try_into().expect("exponent too large for pow()"))
}
*/

pub fn find_modulo(dividend: BigUint, divisor: BigUint) -> BigUint{
    let multiplier = &dividend / &divisor;
    dividend - (multiplier*divisor)
}


pub fn crypt(messages: Vec<u8>, encryptor: u128, modulo: u128) -> Vec<BigUint> {
    let mut k: Vec <BigUint> = Vec::new();
    for message in messages {

        let big_message = message.to_biguint().expect("unable to convert message to BIG INT");
        let big_mod = modulo.to_biguint().expect("unable to convert modulus to BIG INT");

        let c = 1;
        let mut big_c =  c.to_biguint().expect("unable to convert constant to BIG INT");
        for _expo in 0..encryptor{
            let temp_c = &big_message * big_c;
            big_c = find_modulo(temp_c, big_mod.clone());
        }

        /*

        for i in (0..128).map (|n| (encryptor >> n) & 1){

            big_c = &big_c * &big_c;
            big_c = find_modulo(big_c, big_mod.clone());
            if i == 1{
                big_c = &big_c * &big_message;
                big_c = find_modulo(big_c, big_mod.clone());
            }

        }


        let big_temp =pow(big_message ,bit_encryptor);
        println!();
        println!("{}", big_temp);
        big_c = modulo(big_temp, big_mod.clone());
        */

        k.push(big_c);
    }
    k
}