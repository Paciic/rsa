mod cipher;
mod get_primes;
mod modulo;
mod power;

use std::io;
use cipher::crypt;

//TODO: fix jacobi function; fix generator recursion;

fn main() {
    let max_prime_size: u128 = 3456;
    let test = get_primes::primer(max_prime_size);
    println!("{}", test[0]);

    println!("AHHHH");
    println!("{}", test[1]);

    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("what.");
    //conv message to decimal
    let decimal_m: Vec<u8> = m.trim().chars().map(|c| c as u8).collect();
    println!("{}, ", decimal_m[0]);

    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("oopsies");
    //conv key to decimal
    let decimal_e_string: String = e.trim().chars().map(|c| (c as u8).to_string()).collect();

    // Convert the concatenated string into a single integer
    let decimal_e: u128 = decimal_e_string.parse().unwrap();
    println!("{}, ", decimal_e);


    let cipher = crypt(decimal_m, decimal_e, 133);
    for word in cipher {
        let output = &word.to_string();
        print!("{}, ", output);
    }
}
