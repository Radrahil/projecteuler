fn is_prime(x: u32) -> bool {
    let upper_lim = (x as f32).sqrt() as u32;
    let mut prime = true;
    for i in 2..=upper_lim {
        if x % i == 0 {
            prime = false;
            break;
        }
    }
    prime
}

fn main() {
    let mut sum: u64 = 0;
    for i in 2..=2000000 {
        if is_prime(i) {
            sum += i as u64;
        }
    }
    println!("{}", sum);
}
