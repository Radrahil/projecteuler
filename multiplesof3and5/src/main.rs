// Problem 2

fn main() {
    let mut sum: u64 = 0;
    let mut i: u64 = 0;
    while i < 1000 {
        sum += i;
        i += 3;
    }
    i = 0;
    while i < 1000 {
        sum += i;
        i += 5;
    }
    i = 0;
    while i < 1000 {
        sum -= i;
        i += 15;
    }
    println!("{}", sum);
}
