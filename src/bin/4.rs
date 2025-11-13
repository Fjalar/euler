use itertools::Itertools;

fn main() {
    println!(
        "{:?}",
        (100..1000u32)
            .combinations(2)
            .filter_map(|v| {
                let prod = v[0] * v[1];
                let forward = prod.to_string();
                let reverse = forward.chars().rev().collect::<String>();
                if forward == reverse { Some(prod) } else { None }
            })
            .max()
            .unwrap()
    )
}
