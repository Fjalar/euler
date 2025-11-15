use euler::big_u::BigU;

fn main() {
    let mut big = BigU::new(1);
    for i in 1..=100u32 {
        big *= i;
    }
    println!(
        "{}",
        format!("{big}")
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
