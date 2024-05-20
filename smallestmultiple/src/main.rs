// Done using formula a * b = lcm(a*b)*gcd(a*b)

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut lcm: i64 = 1;
    for i in 1..=20 {
        lcm = lcm * i / gcd(lcm, i);
    }
    println!("{}", lcm);
}
