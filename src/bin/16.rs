use euler::big_u::BigU;

fn main() {
    let mut big = BigU::new(1);
    for _ in 0..1000 {
        big *= 2;
    }
    println!(
        "{}",
        format!("{big}")
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
