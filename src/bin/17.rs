fn main() {
    println!("{}", (1..=1000).map(any_num).sum::<u32>());
}

fn single_digit(digit: u32) -> u32 {
    match digit {
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        0 => 0,
        _ => panic!(),
    }
}

fn double_digit(num: u32) -> u32 {
    match num {
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        20 => 6,
        30 => 6,
        40 => 5,
        50 => 5,
        60 => 5,
        70 => 7,
        80 => 6,
        90 => 6,
        0..10 => single_digit(num),
        _ => {
            let ones_digit = num % 10;
            let tens_digit = num - ones_digit;
            double_digit(tens_digit) + single_digit(ones_digit)
        }
    }
}

fn triple_digit(num: u32) -> u32 {
    let tens = num % 100;

    let and = if tens == 0 { 0 } else { 3 };

    let hundreds = num / 100;

    single_digit(hundreds) + 7 + and + double_digit(tens)
}

fn any_num(num: u32) -> u32 {
    match num {
        0 => 4,
        1..10 => single_digit(num),
        10..100 => double_digit(num),
        100..1000 => triple_digit(num),
        1000 => 11,
        _ => panic!(),
    }
}
