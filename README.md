# Simple xor in Rust

## Example

```
let key: Vec<u8> = Vec::from("0x9a");
let buffer: [u8; 512] = [0; 512];

let string_vec = Vec::from("Rustacean");

let key_repeated: Vec<u8> = repeat_key(&key, string_vec.len());
let encoded_bytes : Vec<u8> = buffer.iter().zip(key_repeated).map(|(d, k)| d ^ k).collect();

let key_repeated2: Vec<u8> = repeat_key(&key, string_vec.len() as usize);
let encrypted_bytes : Vec<u8> = string_vec.iter()
.zip(key_repeated2)
.map(|(d, k)| d ^ k)
.collect();

let key_repeated3: Vec<u8> = repeat_key(&key, string_vec.len() as usize);
let decrypted : Vec<u8> = encrypted_bytes.iter()
.zip(key_repeated3)
.map(|(d, k)| d ^ k)
.collect();
```
This will give us the following result when we run the program

```
uhhhh: "0x9a0x9a0"

Whats this?????:
"b\rJ\u{15}Q\u{1b}\\\0^"

decrypted value:
"Rustacean"
```