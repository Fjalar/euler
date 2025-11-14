fn main() {
    let mut index = 1u32;
    let mut triangle = 1u32;

    loop {
        let divisors = (1..=(triangle))
            .filter(|&x| triangle.is_multiple_of(x))
            .count();

        if divisors > 500 {
            println!("{triangle}");
            break;
        }

        index += 1;
        triangle += index;
    }
}
