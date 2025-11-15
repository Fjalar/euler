fn main() {
    let mut longest_chain = 1;
    let mut num_with_longest_chain = 1;

    for i in 1..1_000_000u64 {
        let mut j = i;
        let mut length = 1;
        while j != 1 {
            if j % 2 == 0 {
                j /= 2;
            } else {
                j = 3 * j + 1;
            }
            length += 1;
        }
        if length > longest_chain {
            longest_chain = length;
            num_with_longest_chain = i;
        }
    }

    println!("{num_with_longest_chain}");
}
