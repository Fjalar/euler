fn main() {
    let mut vals = [1, 1];
    let mut offset = 0;
    let mut sum_evens = 0;
    loop {
        let next_offset = offset ^ 1;
        vals[offset] += vals[next_offset];

        if vals[offset] > 4_000_000 {
            println!("{}", sum_evens);
            break;
        } else if vals[offset] % 2 == 0 {
            sum_evens += vals[offset];
        }

        offset = next_offset;
    }
}
