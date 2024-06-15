// Totient function is of the form phi(mn) = phi(m)phi(n)
// Therefore number with most (unique) prime factors will be the answer, since n/phi(n) does not
// increase by multiplying the same number. For example, 3/phi(3) = 3/2, 9 / phi(9) = 3/2
fn is_prime(x: u64) -> bool {
    let upper_lim = (x as f64).sqrt() as u64;
    let mut prime = true;
    for i in 2..=upper_lim {
        if x % i == 0 {
            prime = false;
            break;
        }
    }
    prime
}


fn prime_gen(n: u64) -> Vec<u64> {
    let mut prime_list: Vec<u64> = vec![];
    for i in 2..=n {
        if is_prime(i) {
            prime_list.push(i)
        }
    }
    prime_list
}

fn main() {
    let prime = prime_gen(1000000);
    let mut result = 1;
    for i in prime.into_iter() {
        if result * i > 1000000 {
            break;
        }
        result *= i;
    }
    println!("{result}");
}
