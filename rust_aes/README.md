# Rust AES

I wanted to make a rust implementation of AES for my own interest, so I did.


## Build and test

`cargo build`
`cargo test`


Source - https://github.com/ricmoo/pyaes/tree/23a1b4c0488bd38e03a48120dfda98913f4c87d2.


## Features

 - Raw AES class

 - CBC - Cipher Block Chaining mode
 - CTR - Counter (streaming) mode
 - ECB - Electronic Code Book mode (Do not use. Weak encryption.)
 - Streaming utility (with padding) for Cipher Block Chaining

## Examples - read the tests

### Example of CTR Mode. It is critical to remember and keep secret both the key and initial vector, and remember them to correctly decrypt.

```rust
let key = "lets crypt&*()12".as_bytes().to_vec();
let initial_vector: Vec<u8> = (0..16).map(|_| { rand::random::<u8>() }).collect();

let mut ctr = AESModeOfOperationCTR::new(key.clone(), initial_vector.clone());

// need another ctr with identical state for decoding
let mut ctr2 = AESModeOfOperationCTR::new(key.clone(), initial_vector.clone());


// encrypt some text (we need 2 blocks, so we'll just repeat the same thing twice)
// Its not critical that the plain text be a certain length. This is a stream cipher.
let pt = "lets crypt&*()12".as_bytes().to_vec();
let pt2 = "crypt this".as_bytes().to_vec();
let ct = ctr.encrypt(&pt);
let ct2 = ctr.encrypt(&pt2);

// check the cipher text vs expected text

// decrypt the text

// I'm using 2 blobs (not blocks) that are different plain text content and length.
// Both should have the correctly decoded text.

let decrypted = ctr2.decrypt(&ct);
let decrypted2 = ctr2.decrypt(&ct2);


assert_eq!(pt, decrypted);
assert_eq!(pt2, decrypted2);

```



### Example of CBC Mode. It is critical to keep the key secret and remember it for decryption. It isn't critical to remember the initial vector for decription, but it should be random and secret.

```rust
let key = "lets crypt&*()12".as_bytes().to_vec();
let initial_vector = "AAAAAAAAAAAAAAAA".as_bytes().to_vec();

let mut cbc = AESModeOfOperationCBC::new(key, initial_vector);


// encrypt less than a block and greater than a block
let pt = "lets crypt".as_bytes().to_vec();
let pt2 = "lets crypt&*()12 3456789".as_bytes().to_vec();
let ct = cbc.util_encrypt_stream(&pt);
let ct2 = cbc.util_encrypt_stream(&pt2);


// decrypt the text

// Note how we take off the padding at the front and the back after the decryption here.
let decrypted = cbc.util_decrypt_stream(&ct)[16..16+pt.len()].to_vec();
let decrypted2 = cbc.util_decrypt_stream(&ct2)[16..16+pt2.len()].to_vec();


assert_eq!(pt, decrypted);
assert_eq!(pt2, decrypted2);

```