// Project Euler Problem 3

fn upperlimit (x: &u64) -> u64 {
    let num = *x as f64;
    num.sqrt() as u64
}

fn main() {
    let mut v: Vec<u64> = vec![];
    let n: u64 = 600851475143;
    let ul1: u64 = upperlimit(&n);
    for i in 2..ul1 {
        if n % i == 0 {
            let mut prime: bool = true;
            let ul2 = upperlimit(&i);
            for j in 2..ul2 {
                if i % j == 0 {
                    prime = false;
                    break;
                } 
            }
            if prime {
                v.push(i)
            }
        }
    }
    println!("Largest prime: {}", v.last().unwrap())
}
