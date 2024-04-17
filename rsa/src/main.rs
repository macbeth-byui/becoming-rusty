use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use parsed_args::{ParsedArgs, ArgState};

fn euclid(a:&BigInt, b:&BigInt) -> (BigInt, BigInt, BigInt) {
    if b == &BigInt::zero() {
        return (a.clone(),BigInt::one(),BigInt::zero());
    }
    let (g, i, j) = euclid(b, &(a % b));
    let new_j = &i - (a/b)*&j;
    (g, j, new_j)
}

fn expo_mod(a : &BigInt, b : &BigInt, n : &BigInt) -> BigInt {
    if b == &BigInt::zero() {
        return BigInt::one();
    }
    if b % 2 == BigInt::zero() {
        let c = expo_mod(a, &(b/2), n);
        let power = &c*&c;
        return power % n;
    } else {
        let c = expo_mod(a, &((b-1)/2), n);
        let power = &c*&c*a;
        return power % n;
    }
}

fn init_rsa() -> (BigInt, BigInt, BigInt) {
    let p_int: i128 = 87178291199;
    let q_int: i128 = 22815088913;
    let p = BigInt::from(p_int);
    let q = BigInt::from(q_int);
    let n = &p * &q;
    let r = (&p-1) * (&q-1);
    let e = BigInt::from(65537);
    let (_, i, _) = euclid(&e, &r);
    let d = if i < BigInt::from(0) { (&i + &r) % &r } else { &i % &r };
    (e, d, n)
}

fn enrcypt(data : &BigInt, pub_key : &BigInt, modulo : &BigInt) -> BigInt {
    expo_mod(data, pub_key, modulo)
}

fn decrypt(data : &BigInt, pri_key : &BigInt, modulo : &BigInt) -> BigInt {
    expo_mod(data, pri_key, modulo)
}

fn main() {
    let args = ParsedArgs::new();
    let data = BigInt::from(match args.get_key_arg::<i32>("n") {
       ArgState::Value(value) => value,
       _ => 42
    });
    let (pub_key, pri_key, modulo) = init_rsa();
    println!("pub_key  = {}\npri_key  = {}\nmodulo   = {}", pub_key, pri_key, modulo);
    println!("data     = {}", data);
    let enc_data = enrcypt(&data, &pub_key, &modulo);
    println!("enc data = {}", enc_data);
    let dec_data = decrypt(&enc_data, &pri_key, &modulo);
    println!("dec data = {}", dec_data);
}