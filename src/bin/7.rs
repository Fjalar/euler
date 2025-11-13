fn main() {
    let mut primes = vec![2];

    let mut x = 3;

    'outer: loop {
        for p in &primes {
            if x % p == 0 {
                x += 2;
                continue 'outer;
            }
        }
        if primes.len() == 10_001 {
            println!("{}", primes[10_000]);
            break;
        }
        primes.push(x);
        x += 2;
    }
}
