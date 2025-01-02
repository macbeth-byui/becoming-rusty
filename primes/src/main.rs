
fn is_prime(n : u64, seed : Option<&Vec<u64>>) -> bool {
    let mut d = match seed {
        None => 3,
        Some(seed) => {
            for p in seed {
                if p * p > n {
                    return true;
                }
                if n % p == 0 {
                    return false;
                }
            }
            seed.last().unwrap() + 2
        }
    };
    loop {
        if d * d > n {
            return true;
        }
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
}

fn main() {
    let mut curr : u64 = 2;
    let mut count : u64 = 0;
    let mut millions : u64 = 0;
    let mut seed : Vec<u64> = Vec::new();

    loop {
        if is_prime(curr,None) {
            seed.push(curr);
            if seed.len() == 10000000 {
                break;
            }
        }
        curr += 1;
    }

    println!("Prime Seed: {:?} - {:?}", seed.first(), seed.last());
    // println!("Seed: {:?}", seed);

    println!("Searching for Primes");
    curr = 2;
    loop {
        if is_prime(curr,Some(&seed)) {
            count += 1;
            if count == 1_000_000 {
                millions += 1;
                count = 0;
                println!("Found {millions} million primes.  Current prime is {curr}");
            }    
        }
        curr += 1;
    }
}
