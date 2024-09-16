mod cipher;
mod get_primes;
mod modulo;
mod power;

use std::io;
use cipher::crypt;

//TODO: fix jacobi function; fix generator recursion;

fn main() {
    let max_prime_size: u128 = 500;
    let test = get_primes::primer(max_prime_size);
    println!("{}", test[0]);
    println!("AHHHH");
    println!("{} </end_of_the_very_messy_section>", test[1]);

    //MESSAGE
    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("what.");
    //conv message to decimal
    let decimal_m: Vec<u8> = m.trim().chars().map(|c| c as u8).collect();
    println!("{}, ", decimal_m[0]);



    //KEY (to be replaced by prime function generator via phi(n) i think)
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("oopsies");
    //conv key to decimal
    let decimal_e_string: String = e.trim().chars().map(|c| (c as u8).to_string()).collect();

    // Convert the concatenated string into a single integer
    let decimal_e: u128 = decimal_e_string.parse().unwrap();
    println!("{}, ", decimal_e);

    /* to be repalced by only message,
    prime generator will be auto,
    e will be calculated by phi(n) <- prime number generator */
    let cipher = crypt(decimal_m);
    for word in cipher {
        let output = &word.to_string();
        print!("{}, ", output);
    }
}
