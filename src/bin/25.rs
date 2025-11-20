#![feature(f128)]

fn main() {
    let mut vals: [f128; 2] = [1.0, 1.0];
    let mut idx = 3;
    loop {
        vals[(idx + 1) % 2] += vals[idx % 2];

        if vals[(idx + 1) % 2] > 1e999 {
            break;
        }

        idx += 1;
    }

    println!("{idx}");
}
