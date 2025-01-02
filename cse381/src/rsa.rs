use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};

/* Convienant storage of RsaKeys
 */
#[derive(Debug)]
pub struct RsaKeys {
    pub pub_key : BigInt,
    pub pri_key : BigInt,
    pub mod_value : BigInt
}


/* Determine the greatest common divisor of a and b.
 * The answer is given in the format (gcd, i, j) where
 * gcd = i*a + j*b
 */
fn euclid(a : &BigInt, b : &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        return (a.clone(), BigInt::one(), BigInt::zero())
    }
    let (gcd, i, j) = euclid(b, &(a % b));
    (gcd, j.clone(), i - (a / b) * j)
}

/* Determine the value of (x^y) mod n
 */
fn mod_expo(x : &BigInt, y : &BigInt, n : &BigInt) -> BigInt {
    if y.is_zero() {
        return BigInt::one();
    }
    if y % 2 == BigInt::zero() {
        let z = mod_expo(x, &(y / 2), n);
        return z.pow(2) % n;
    }
    let z = mod_expo(x, &((y - 1) / 2), n);
    (z.pow(2) * x) % n
}

/* Find a random value that is relatively prime to x and within 
 * the range of 3 to x-1.  This does involve a loop to guess
 * relatively prime numbers.  However, the odds are in our favor.
 */
fn rand_rel_prime(x : &BigInt) -> BigInt {
    let mut gen = rand::thread_rng();
    loop {
        let y = gen.gen_bigint_range(&BigInt::from(3), x);
        // Verify that the randomly selected value really in 
        // relatively prime to x by using euclid.
        let (gcd, _, _) = euclid(x, &y);
        if gcd == BigInt::one() {
            return y;
        }
    }
}

/* Perform a mod operation but ensure the answer is positive
 */
fn mod_neg(a : &BigInt, b : &BigInt) -> BigInt {
    ((a % b) + b) % b
}

/* Create rsa keys based on two prime numbers p and q
 */
pub fn gen_keys(p : &BigInt, q : &BigInt) -> RsaKeys {
    let n = p * q;
    let phi = (p - BigInt::one()) * (q - BigInt::one());
    let e = rand_rel_prime(&phi);

    // Calculate the multiplicative inverse of e mod phi
    let (_, i, _) = euclid(&e, &phi);
    let d = mod_neg(&i, &phi);
    
    RsaKeys {pub_key : e, pri_key : d, mod_value : n}
}

/* Encrypt the value using the public key
 */
pub fn encrypt(value : &BigInt, keys : &RsaKeys) -> BigInt {
    mod_expo(value, &keys.pub_key, &keys.mod_value)
}

/* Decrypt the value using the private key
 */
pub fn decyprt(value : &BigInt, keys : &RsaKeys) -> BigInt {
    mod_expo(value, &keys.pri_key, &keys.mod_value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_euclid_small() {
        let a = BigInt::from(8);
        let b = BigInt::from(12);

        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(4));
        assert_eq!(i, BigInt::from(-1));
        assert_eq!(j, BigInt::from(1));
    }

    #[test]
    fn test2_euclid_coprime_small() {
        let a = BigInt::from(5);
        let b = BigInt::from(72);

        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, BigInt::from(29));
        assert_eq!(j, BigInt::from(-2)); 
    }

    #[test]
    fn test3_euclid_coprime_big() {
        let a = BigInt::from(65537);
        let b = ("87178291199".parse::<BigInt>().unwrap() - 1) * 
                        ("22815088913".parse::<BigInt>().unwrap() - 1);
        
        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, "-691197798001282429727".parse::<BigInt>().unwrap());
        assert_eq!(j, BigInt::from(22775)); 
    }

    #[test]
    fn test4_mod_expo() {
        let a = BigInt::from(3);
        let b = BigInt::from(50);
        let n = BigInt::from(5);
        let result = mod_expo(&a, &b, &n);
        assert_eq!(result, BigInt::from(4));
    }

    #[test]
    fn test5_rand_rel_prime() {
        let phi = ("87178291199".parse::<BigInt>().unwrap() - 1) * 
                          ("22815088913".parse::<BigInt>().unwrap() - 1);
        let e = rand_rel_prime(&phi);
        let (gcd, _, _) = euclid(&e, &phi);
        assert_eq!(gcd, BigInt::one());
    }

    #[test]
    fn test6_encrypt_decrypt() {
        let p = "87178291199".parse::<BigInt>().unwrap();
        let q = "22815088913".parse::<BigInt>().unwrap();
        let keys = gen_keys(&p, &q);
        let value = BigInt::from(42);
        let encrypted = encrypt(&value, &keys);
        let decrypted = decyprt(&encrypted, &keys);
        assert_eq!(decrypted, value);
    }

    #[test]
    fn test7_encrypt_decrypt() {
        let p = "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123".parse::<BigInt>().unwrap();
        let q = "531872289054204184185084734375133399408303613982130856645299464930952178606045848877129147820387996428175564228204785846141207532462936339834139412401975338705794646595487324365194792822189473092273993580587964571659678084484152603881094176995594813302284232006001752128168901293560051833646881436219".parse::<BigInt>().unwrap();
        let keys = gen_keys(&p, &q);
        let value = BigInt::from(42);
        let encrypted = encrypt(&value, &keys);
        let decrypted = decyprt(&encrypted, &keys);
        assert_eq!(decrypted, value);
    }

    #[test]
    fn test8_encrypt_decrypt() {
        let p = "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123".parse::<BigInt>().unwrap();
        let q = "531872289054204184185084734375133399408303613982130856645299464930952178606045848877129147820387996428175564228204785846141207532462936339834139412401975338705794646595487324365194792822189473092273993580587964571659678084484152603881094176995594813302284232006001752128168901293560051833646881436219".parse::<BigInt>().unwrap();
        let keys = gen_keys(&p, &q);
        let value = BigInt::from(42);
        let encrypted = encrypt(&value, &keys);
        let decrypted = decyprt(&encrypted, &keys);
        assert_eq!(decrypted, value);
    }

}
