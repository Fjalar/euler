fn main() {
    let sum_of_squares = (1..=100u64).map(|x| x.pow(2)).sum::<u64>();
    let square_of_sum = (1..=100u64).sum::<u64>().pow(2);

    println!("{}", square_of_sum - sum_of_squares);
}
