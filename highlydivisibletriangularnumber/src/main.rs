// Inefficient Solution
//fn number_of_divisors(x: u64) -> u64 {
//    let mut divisors: u64 = 0;
//    let lim = (x as f64).sqrt() as u64;
//    for i in 1..=lim {
//        if x % i == 0 {
//            divisors += 2;
//        }
//    }
//    // If x is a perfect square, we've counted root x twice
//    if lim * lim == x {
//        divisors -= 1;
//    }
//    divisors
//}

fn main() {
    let prime_list = prime_gen(100_000); // Generate primes up to 100,000
    let mut n: u64 = 1;
    let mut triangle_number: u64 = 1;

    loop {
        let div = number_of_divisors(triangle_number, &prime_list);
        if div > 500 {
            println!("Triangle Number: {}, Divisors: {}", triangle_number, div);
            break;
        }
        n += 1;
        triangle_number += n;
    }
}

// Function to calculate the number of divisors of x using its prime factorization
fn number_of_divisors(mut x: u64, prime_list: &[u64]) -> u64 {
    //slice will do instead of borrowed vector ^
    let mut divisors: u64 = 1;

    for &prime in prime_list.iter() {
        if prime * prime > x {
            break;
        }
        let mut count = 0;
        while x % prime == 0 {
            x /= prime;
            count += 1;
        }
        divisors *= count + 1;
    }

    // If there's any remaining prime factor
    if x > 1 {
        divisors *= 2;
    }

    divisors
}

// Sieve of Eratosthenes to generate all prime numbers up to 'limit'
fn prime_gen(limit: usize) -> Vec<u64> {
    let mut sieve = vec![true; limit + 1];
    let mut primes = Vec::new();

    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=limit {
        if sieve[i] {
            primes.push(i as u64);
            let mut multiple = i * i;
            while multiple <= limit {
                sieve[multiple] = false;
                multiple += i;
            }
        }
    }

    primes
}
