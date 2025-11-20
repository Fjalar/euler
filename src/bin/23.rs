use intset::ShrinkSet;

fn main() {
    let mut constructible_set = ShrinkSet::new(28_123 + 1);

    let mut abundant_set = Vec::new();

    for i in 1..28_123u64 {
        let divisor_sum = (1..=(i / 2))
            .filter(|&div| i.is_multiple_of(div))
            .sum::<u64>();
        if divisor_sum > i {
            abundant_set.push(i);
        }
    }

    for i in &abundant_set {
        for j in &abundant_set {
            constructible_set.remove((i + j) as usize);
        }
    }
    println!("{}", constructible_set.iter().sum::<usize>());
}
