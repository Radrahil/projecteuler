// If either a or b >= 500 then c must be less than 500, which is simply not possible

fn main() {
    for a in 1..500 {
        for b in 1..500 {
            let c = 1000 - (a + b);
            if c * c == a * a + b * b {
                println!("{a} {b} {c}");
            }
        }
    }
}
