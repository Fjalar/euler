fn main() {
    // primes calculation from 10.rs
    let mut primes = vec![2u32];
    let mut x = 3u32;

    let mut index = 1u32;
    let mut triangle = 1u32;

    loop {
        let divisors = primes
            .iter()
            .filter(|&x| triangle.is_multiple_of(*x))
            .map(|divisor| {
                let mut tri = triangle;
                let mut count = 0;
                // https://en.wikipedia.org/wiki/Divisor_function#Properties
                // numer of divisors = Product_n((prime_factor_exponent_n + 1))
                // number of divisors = (prime_factor_exponent_1 + 1)(prime_factor_exponent_2 + 1)(prime_factor_exponent_3 +1)(...)
                while tri.is_multiple_of(*divisor) {
                    count += 1;
                    tri /= divisor;
                }
                count + 1
            })
            .product::<u32>();

        if divisors > 500 {
            println!("{triangle}");
            break;
        }

        index += 1;
        triangle += index;

        'outer: while triangle.isqrt() > *primes.last().unwrap() {
            for p in &primes {
                if x.is_multiple_of(*p) {
                    x += 2;
                    continue 'outer;
                }
            }
            primes.push(x);
            x += 2;
        }
    }
}
