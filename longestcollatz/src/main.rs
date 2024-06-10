use std::collections::HashMap;

// collatz fn
fn collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        (3 * n) + 1
    }
}

// the main chain fn
// used memoisation (DP) to not recompute.
// Time complexity is O(N). (is time complexity without memoisation O(N!)?)

fn collatz_chain(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 1 {
        return 1;
    }

    //HashMap is the better choice here instead of array.
    if let Some(&length) = memo.get(&n) {
        return length;
    }
    let length = 1 + collatz_chain(collatz(n), memo);
    memo.insert(n, length);
    length
}

fn main() {
    let mut memo = HashMap::new();
    let mut longest_chain_length = 0;
    let mut number_with_longest_chain = 0;

    for i in 1..=1_000_000 {
        let current_chain_length = collatz_chain(i, &mut memo);
        if current_chain_length > longest_chain_length {
            longest_chain_length = current_chain_length;
            number_with_longest_chain = i;
        }
    }

    println!(
        "Number with the longest chain: {}",
        number_with_longest_chain
    );
    println!("Length of the longest chain: {}", longest_chain_length);
}
