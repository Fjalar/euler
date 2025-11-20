use itertools::Itertools;

fn main() {
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .iter()
        .permutations(10)
        .sorted()
        .take(1_000_000)
        .next_back()
        .unwrap()
        .iter()
        .for_each(|n| print!("{n}"));
}
