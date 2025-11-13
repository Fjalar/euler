fn main() {
    let mut max = 0;

    for a in 1..1000u32 {
        for b in a..1000u32 {
            if (a + b) > 999 {
                break;
            }
            let c = 1000u32 - a - b;

            let a_sq = a.pow(2);
            let b_sq = b.pow(2);
            let c_sq = c.pow(2);

            if a_sq + b_sq == c_sq || a_sq + c_sq == b_sq || b_sq + c_sq == a_sq && a * b * c > max
            {
                max = a * b * c;
            }
        }
    }

    println!("{}", max);
}
