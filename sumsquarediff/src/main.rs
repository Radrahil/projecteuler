// To find the diff between (1 + 2 + 3 ... + 100)^2 and 1^2 + 2^2 + 3^2 + ... + 100^2

fn main() {
    let sum: u64 = 100*101/2;
    let sum_square: u64 = sum * sum;
    let indi_square: u64 = 100*101*201/6;

    let diff = sum_square - indi_square;

    println!("{}", diff);
}
