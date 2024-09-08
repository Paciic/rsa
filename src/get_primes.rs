
extern crate rand;
use rand::thread_rng;
use rand::RngCore;
use crate::modulo::find_modulo;
use num_bigint::{BigUint, ToBigUint, RandBigInt, ToBigInt, BigInt};
use crate::power::pow;

//max min funcs
fn max(a:BigUint ,b:BigUint) -> BigUint{
    if a > b{
        return a;
    }
    else {
        return b;
    }
}
fn min(a:BigUint ,b:BigUint) -> BigUint{
    if a > b{
        b
    }
    else {
        a
    }
}


#[allow(non_snake_case)]
fn J(a:BigUint, n: BigUint) -> bool{
    if &n<=&0.to_biguint().unwrap() || &n % &2.to_biguint().unwrap() != 1.to_biguint().unwrap(){
        return false;
    }
    let mut november = n.clone();
    let mut alpha = &a % &n.clone();
    let mut t:BigInt = 1.to_bigint().unwrap();
    let mut r:BigUint;
    while &alpha != &0.to_biguint().unwrap() {

        while &a % 2.to_biguint().unwrap() == 0.to_biguint().unwrap() {
            alpha = alpha / 2.to_biguint().unwrap();
            r = &november % 8.to_biguint().unwrap();
            if r == 3.to_biguint().unwrap() || r == 5.to_biguint().unwrap() {
                t = -t;
            }
        }
            r = november;
            november = alpha;
            alpha = r;
            if &a % 4.to_biguint().unwrap() == 3.to_biguint().unwrap()
                &&
                &n % 4.to_biguint().unwrap() == 3.to_biguint().unwrap() {
                t = -t;
            }
        alpha = alpha % &november;
    }
    if november != 1.to_biguint().unwrap(){
        return false
    }
    let exp = (&n - 1.to_biguint().unwrap())/2.to_biguint().unwrap();
    let divisor = pow(a.clone(), exp.clone());
    let result = find_modulo(&divisor, &n);
    if result < 0.to_biguint().unwrap(){
        return false;
    }
    let integer_result = result.to_bigint().unwrap();

    if integer_result == t{
        return true;
    }
    false

}
fn gcd(a:BigUint, b:BigUint) -> bool{
    let mut r:BigUint = 1.to_biguint().unwrap();
    let mut x=
        max(a.clone(),b.clone());
    let mut y =
        min(a,b);

    while r != 0.to_biguint().unwrap(){
        r = find_modulo(&x, &y);
        x = y.clone();
        y = r.clone();
    }

    if y == 1.to_biguint().unwrap(){
        println!("is prime");
        return true;
    }
    false
}

fn check_primes(a:BigUint, b:BigUint) -> bool{
    //we now follow the 1977 "A Method for Obtaining Digital Signatures and Public-Key Cryptosystems"'s methodology to determine if a and b are coprime
    if gcd(a.clone(), b.clone()) && J(a.clone() ,b.clone() ){
        return true
    }
    return false
}

pub fn get_primes(max: u128) -> Vec<BigUint>{
    //get random seed to find b and a
    let big_max = max.to_biguint().expect("Failed to convert into big unit");

    let mut temp_seed = thread_rng();
    let seed = RandBigInt::gen_biguint_range(&mut temp_seed, &0.to_biguint().expect("how the hell did you mess this up") ,&big_max);

    let b =
    //do not ask.
        &seed
        * 2.to_biguint().expect("how")
        + 1.to_biguint().expect("what");

    //random uniform distribution between {1...b-1} where a % 2 != 0
    let mut temp_a = thread_rng();
    let a = RandBigInt::gen_biguint_range(&mut temp_a, &0.to_biguint().expect("how the hell did you mess this up...part 2??") ,&seed)
        //do not ask again.
        * 2.to_biguint().expect("how")
        + 1.to_biguint().expect("what");


    let result = vec![a, b];
    return result;

}

pub fn primer(max_prime_size: u128) -> Vec<BigUint>{
    let mut counter = 0;
    let mut passer: Vec<BigUint> = Vec::new();
    while counter <= 3{
        let ab= get_primes(max_prime_size);
        let a = ab[0].clone();
        let b = ab[1].clone();
        if check_primes(a ,b) {
            counter = counter + 1;
        } else {
            counter = 0;
        }
        if counter == 3{
            passer = ab;
        }
    }
    return passer;

}

