fn main() {
    let primes = (3..2_000_000u64).step_by(2).filter(|&num| {
        let mut divisor = 3u64;
        while (divisor * divisor) < 2_000_000u64 && divisor < num {
            if num.is_multiple_of(divisor) {
                return false;
            };
            divisor += 2;
        }
        true
    });
    println!("{}", primes.sum::<u64>() + 2);
}
