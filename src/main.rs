use std::collections::HashMap;
use std::fs::read_dir;
use std::process::Command;
use std::time::Instant;

fn main() {
    let all = read_dir("./src/bin/")
        .unwrap()
        .map(|dir| dir.unwrap().file_name().into_string().unwrap());

    _ = Command::new("cargo build --release --bin").spawn();

    let mut times = Vec::<(f32, String)>::new();

    let start_all = Instant::now();
    for name in all {
        println!("Running {name}:");
        let relative_path = ["./target/release/", name.strip_suffix(".rs").unwrap()].join("");
        let before = Instant::now();
        println!(
            "{}",
            String::from_utf8(Command::new(relative_path).output().unwrap().stdout).unwrap()
        );
        times.push((before.elapsed().as_secs_f32(), name.clone()));
    }

    let total_time = start_all.elapsed().as_secs_f32();

    println!(
        "Took {:.0}min {}s seconds for all available solutions to complete",
        total_time / 60.0,
        total_time % 60.0
    );

    times.sort_by(|(time1, _), (time2, _)| time2.total_cmp(time1));

    println!("Time in descending order: ");
    for (time, name) in times {
        println!("{name}: {:.0}min {}s", time / 60.0, time % 60.0);
    }
}
