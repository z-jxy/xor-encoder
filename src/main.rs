fn main() {
    let key: Vec<u8> = Vec::from("0x9a");
    let buffer: [u8; 512] = [0; 512];

    let string_vec = Vec::from("Rustacean");

    let key_repeated: Vec<u8> = repeat_key(&key, string_vec.len());
    let encoded_bytes : Vec<u8> = buffer.iter().zip(key_repeated).map(|(d, k)| d ^ k).collect();
    println!("uhhhh: {:#?}", String::from_utf8(encoded_bytes.to_vec()).unwrap());

    // Iterating over encoded string
    let key_repeated2: Vec<u8> = repeat_key(&key, string_vec.len() as usize);
    let encrypted_bytes : Vec<u8> = string_vec.iter()
    .zip(key_repeated2)
    .map(|(d, k)| d ^ k)
    .collect();
    
    // Iterating over encrypted bytes
    let key_repeated3: Vec<u8> = repeat_key(&key, string_vec.len() as usize);
    let decrypted : Vec<u8> = encrypted_bytes.iter()
    .zip(key_repeated3)
    .map(|(d, k)| d ^ k)
    .collect();

    println!("whats this?????: \n{:#?}", String::from_utf8(encrypted_bytes).unwrap());

    println!("decrypted value: \n{:#?}", String::from_utf8(decrypted).unwrap());

    }

fn repeat_key(key : &Vec<u8>, required_len : usize) -> Vec<u8> {
    let mut key_repeated = Vec::with_capacity(required_len);
    while key_repeated.len() < required_len {
        for &b in key {
            key_repeated.push(b);
            if key_repeated.len() == required_len {
                break;
            }
        }
    }
    key_repeated
}
