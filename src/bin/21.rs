use std::collections::BTreeSet;

fn main() {
    let mut amicable_set = BTreeSet::new();

    for i in (1..10_000u64) {
        if amicable_set.contains(&i) {
            continue;
        }
        let divisor_sum = (1..=(i / 2))
            .filter(|&div| i.is_multiple_of(div))
            .sum::<u64>();
        if divisor_sum == i || amicable_set.contains(&divisor_sum) {
            continue;
        }
        let other_sum = (1..=(divisor_sum / 2))
            .filter(|&div| divisor_sum.is_multiple_of(div))
            .sum::<u64>();
        if i == other_sum {
            amicable_set.insert(i);
            amicable_set.insert(divisor_sum);
        }
    }

    println!("{}", amicable_set.iter().sum::<u64>());
}
