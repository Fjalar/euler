fn main() {
    println!(
        "{}",
        (0u32..1000)
            .filter(|i| i % 3 == 0 || i % 5 == 0)
            .sum::<u32>()
    );
}
