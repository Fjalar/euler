fn main() {
    let mut primes = vec![2];

    let target_numerator = 600_851_475_143u64;

    let upper_bound = (target_numerator as f64).sqrt() as u64;

    let mut factors = vec![];

    'outer: for x in 3..upper_bound {
        for p in &primes {
            if x % p == 0 {
                continue 'outer;
            }
        }
        if target_numerator.is_multiple_of(x) {
            factors.push(x);
            if factors.iter().product::<u64>() == target_numerator {
                println!("{}", factors.last().unwrap());
                break;
            }
        }
        primes.push(x);
    }
}
