use std::fs::read_dir;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let all = read_dir("./src/bin/")
        .unwrap()
        .map(|dir| dir.unwrap().file_name().into_string().unwrap());

    println!(
        "{}",
        String::from_utf8(
            Command::new("cargo")
                .args(["build", "--release", "--bins"])
                .output()
                .unwrap()
                .stdout
        )
        .unwrap()
    );

    let mut times = Vec::<(Duration, String)>::new();

    let start_all = Instant::now();
    for name in all {
        println!("Running {name}:");
        let relative_path = ["./target/release/", name.strip_suffix(".rs").unwrap()].join("");
        let before = Instant::now();
        println!(
            "{}",
            String::from_utf8(Command::new(relative_path).output().unwrap().stdout).unwrap()
        );
        times.push((before.elapsed(), name.clone()));
    }

    let total_time = start_all.elapsed();

    let mins = total_time.as_secs() / 60;
    let secs = total_time.as_secs() - mins * 60;
    let millis = total_time.as_millis() % 1000;
    let micros = total_time.as_micros() % 1000;

    println!(
        "Took {mins}min {secs}s {millis}ms {micros:3.}us for all available solutions to complete"
    );

    times.sort_by(|(time1, _), (time2, _)| time2.cmp(time1));

    println!("Time in descending order: ");
    for (time, name) in times {
        let mins = time.as_secs() / 60;
        let secs = time.as_secs() - mins * 60;
        let millis = time.as_millis() % 1000;
        let micros = time.as_micros() % 1000;
        println!("{name:>6}: {mins:>4}min {secs:>4}s {millis:>4}ms {micros:3.}us");
    }
}
