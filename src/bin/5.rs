fn main() {
    let mut x = 20;
    'outer: loop {
        for divisor in 9..20 {
            if x % divisor != 0 {
                x += 20;
                continue 'outer;
            }
        }
        break;
    }
    println!("{}", x);
}
