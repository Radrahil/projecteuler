fn main() {
    let mut n: i64 = 0;
    for i in 1.. {
        n += i;
        let div = number_of_divisors(n);
        if div > 500 {
            println!("{}, {}", n, div);
            break;
        }
    }
}

fn number_of_divisors(x: i64) -> i32 {
    let mut divisors: i32 = 0;
    let lim = (x as f64).sqrt() as i64;
    for i in 1..=lim {
        if x % i == 0 {
            divisors += 2;
        }
    }
    // If x is a perfect square, we've counted root x twice
    if lim * lim == x {
        divisors -= 1;
    }
    divisors
}
