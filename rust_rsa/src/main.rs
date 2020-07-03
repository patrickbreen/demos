extern crate num_bigint;
extern crate rand;

use num_bigint::{ToBigUint, BigUint, RandBigInt};


fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {

    while &b != &0.to_biguint().unwrap() {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    return a;
}

fn multiplicative_inverse_mod(e: BigUint, phi: BigUint) -> BigUint {

    let mut d = 1_u32.to_biguint().unwrap();
    let step = 1_u32.to_biguint().unwrap();

    if gcd(e.clone(), phi.clone()) != 1.to_biguint().unwrap() {
        panic!("e and phi must be co-prime!!");
    }

    while true {
        let ed_mod_phi = &e * &d % &phi;

        if ed_mod_phi == 1_u32.to_biguint().unwrap() {
            return d;
        } else {
            d += &step;
        }
    }
    panic!("this never happens");
}

fn is_prime(n: &BigUint) -> bool {
    if n == &2_u32.to_biguint().unwrap() {
        return true;
    } else if n < &2_u32.to_biguint().unwrap() || n % &2_u32.to_biguint().unwrap() == 0.to_biguint().unwrap() {
        return false;
    }

    let mut i = 3_u32.to_biguint().unwrap();
    let end = n.sqrt()+2_u64.to_biguint().unwrap();
    let step = 2_u32.to_biguint().unwrap();


    while &i < &end {
        if n % &i == 0.to_biguint().unwrap() {
            return false;
        }
        i += &step;
    }
    true
}

fn generate_keypair(p: BigUint, q: BigUint) -> (num_bigint::BigUint, num_bigint::BigUint, num_bigint::BigUint, num_bigint::BigUint) {
    
    // note, key generation currently takes way too long for this to be remotely viable


    if !(is_prime(&p) && is_prime(&q)) {
        panic!("both p and q must be prime");
    } else if p == q {
        panic!("p cannot equal q");
    }

    let n = &p * &q;
    let phi = (&p-&1.to_biguint().unwrap()) * (&q-&1.to_biguint().unwrap());

    let mut rng = rand::thread_rng();
    let mut e = rng.gen_biguint_range(&2.to_biguint().unwrap(), &phi);
    let mut g1 = gcd(e.clone(), phi.clone());
    let mut g2 = gcd(e.clone(), n.clone());

    while !(&g1 == &1.to_biguint().unwrap() && &g2 == &1.to_biguint().unwrap()) {
        e = rng.gen_biguint_range(&1.to_biguint().unwrap(), &phi);
        g1 = gcd(e.clone(), phi.clone());
        g2 = gcd(e.clone(), n.clone());
    }

    let d = multiplicative_inverse_mod(e.clone(), phi.clone());

    return (e, n.clone(), d, n);
}

fn encrypt(key: &BigUint, n: &BigUint, plaintext: &BigUint) -> num_bigint::BigUint {
    return plaintext.modpow(&key, &n);
}

fn decrypt(key: &BigUint, n: &BigUint, ciphertext: &BigUint) -> num_bigint::BigUint {
    return ciphertext.modpow(&key, &n);
}


fn main() {
    println!("Hello, world!");
}


mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test_prime() {

        assert!(!is_prime(&45_u32.to_biguint().unwrap()));
        assert!(is_prime(&29_u32.to_biguint().unwrap()));
    }

    #[test]
    fn test_gcd() {

        let mut ans = gcd(14_u32.to_biguint().unwrap(), 21_u32.to_biguint().unwrap());
        assert!(ans == 7_u32.to_biguint().unwrap());


        ans = gcd(11_u32.to_biguint().unwrap(), 55_u32.to_biguint().unwrap());
        assert!(ans == 11_u32.to_biguint().unwrap());


        ans = gcd(9_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        assert!(ans == 3_u32.to_biguint().unwrap());

        ans = gcd(3_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        assert!(ans == 3_u32.to_biguint().unwrap());
    }


    #[test]
    #[should_panic]
    fn test_multiplicative_inverse_panic() {
        let mut ans = multiplicative_inverse_mod(9_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        println!("ans: {:?}", ans);
        assert!(ans == 5_u32.to_biguint().unwrap());
    }

    #[test]
    fn test_multiplicative_inverse_success() {

        let mut ans = multiplicative_inverse_mod(5_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        assert!(ans == 5_u32.to_biguint().unwrap());

        let mut ans = multiplicative_inverse_mod(11_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        assert!(ans == 5_u32.to_biguint().unwrap());

        let mut ans = multiplicative_inverse_mod(13_u32.to_biguint().unwrap(), 6_u32.to_biguint().unwrap());
        assert!(ans == 1_u32.to_biguint().unwrap());


        let mut ans = multiplicative_inverse_mod(7_u32.to_biguint().unwrap(), 10_u32.to_biguint().unwrap());
        assert!(ans == 3_u32.to_biguint().unwrap());
    }



    #[test]
    fn test_generate_key_encrypt_decrypt_known_p_and_q() {

        let secret_plaintext = 3_u32.to_biguint().unwrap();
        let p = 2_u32.to_biguint().unwrap();
        let q = 7_u32.to_biguint().unwrap();


        let (e, n1, d, n2) = generate_keypair(p, q);

        let secret_ciphertext = encrypt(&e, &n1, &secret_plaintext);

        // encryption and decryption are the same operation with different keys
        let decrypted_plaintext = encrypt(&d, &n1, &secret_ciphertext);

        assert!(&secret_plaintext == &decrypted_plaintext);
    }



    #[test]
    fn test_generate_key_encrypt_decrypt_big_p_and_q() {

        let secret_plaintext = 3_u32.to_biguint().unwrap();
        let p = 701.to_biguint().unwrap();
        let q = 919.to_biguint().unwrap();


        let (e, n1, d, n2) = generate_keypair(p, q);

        let secret_ciphertext = encrypt(&e, &n1, &secret_plaintext);

        // encryption and decryption are the same operation with different keys
        let decrypted_plaintext = encrypt(&d, &n1, &secret_ciphertext);

        assert!(&secret_plaintext == &decrypted_plaintext);
    }
}