use crate::modulo::find_u128i32_modulo;

pub fn u128tobin(number: u128) -> Vec<bool>{
    let mut result:Vec<bool> = Vec::new();
    for power in (0..128){
        let new_number = find_u128i32_modulo(&number, &2 ^ power);

    }
    result
}