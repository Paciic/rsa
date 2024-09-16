
extern crate rand;
use rand::thread_rng;
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
//TODO: FIX THIS PIECE OF SHIT.
    let mut t = 1.to_biguint().unwrap();


    //TODO: remove.
    true

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

    if x == 1.to_biguint().unwrap(){
        println!("gcd is prime");
        return true;
    }
    false
}

fn check_primes(a:BigUint, b:BigUint) -> bool{
    //we now follow the 1977 "A Method for Obtaining Digital Signatures and Public-Key Cryptosystems"'s methodology to determine if a and b are coprime
    if gcd(a.clone(), b.clone()) && (J(a.clone() ,b.clone())){
        return true
    }
    return false
}

pub fn get_primes(max: u128) -> Vec<BigUint>{
    //get random seed to find b and a
    let big_seed_min = 1.to_biguint().expect("how the hell did you mess this up");
    let big_max = max.to_biguint().expect("Failed to convert into big unit");

    let mut temp_seed = thread_rng();
    let seed = RandBigInt::gen_biguint_range(&mut temp_seed, & big_seed_min,&big_max);

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
    let mut pass = false;
    let mut passer: Vec<BigUint> = Vec::new();

    while pass == false{
        if counter == 0{
            //passer = get_primes(max_prime_size);

            //TODO: fix prime generator (probably borked)
            passer.push(167.to_biguint().unwrap());
            passer.push(317.to_biguint().unwrap());
            println!("prime1: {}", passer[0]);
            println!("prime2: {}", passer[1])
        }
        let combab= &passer;
        let a = combab[0].clone();
        let b = combab[1].clone();
        if check_primes(a ,b) {
            counter = counter + 1;
        } else {
            counter = 0;
        }
        if counter == 3{
            pass = true;
        }
    }
    return passer;
}

