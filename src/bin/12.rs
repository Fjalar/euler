fn main() {
    let mut index = 1u32;
    let mut triangle = 1u32;

    let mut max = 0;

    loop {
        let divisors = (1..=(triangle))
            .filter(|&x| triangle.is_multiple_of(x))
            .count();

        if divisors > max {
            max = divisors;
            println!("{triangle}: {max}");
        }

        if divisors > 500 {
            println!("{triangle}");
            break;
        }

        index += 1;
        triangle += index;
    }
}
