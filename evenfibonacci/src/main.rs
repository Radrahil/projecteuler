fn main() {
    let mut sum: u64 = 0;
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = 1;

    while c < 4000000 {
        if c % 2 == 0 {
            sum += c;
        }
        c = a + b;
        a = b;
        b = c;
    }
    println!("{}", sum);
}
