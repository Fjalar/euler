fn main() {
    let mut primes = vec![2u64];

    let mut x = 3u64;

    'outer: loop {
        if x > 2_000_000 {
            break;
        }
        for p in &primes {
            if x.is_multiple_of(*p) {
                x += 2;
                continue 'outer;
            }
        }
        primes.push(x);
        x += 2;
    }

    println!("{}", primes.iter().sum::<u64>());
}
