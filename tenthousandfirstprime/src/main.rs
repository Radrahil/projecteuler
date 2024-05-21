fn upper_limit(a: u64) -> u64 {
    let x = a as f64;
    x.sqrt() as u64
}

fn is_prime(n: u64) -> bool {
    let u = upper_limit(n);

    for i in 2..=u {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut prime_count = 0;
    let mut i: u64 = 1;

    while prime_count < 10001 {
        i += 1;
        if is_prime(i) {
            prime_count += 1;
        }
    }
    println!("{}", i);
    // Answer is 104743. Feels abnormally small.
}
