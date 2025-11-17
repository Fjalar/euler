# Project Euler
Project Euler solutions written in Rust, with benchmarking

## Benchmark

Executing `cargo run` will run all available solutions in release mode, time each individual solution, as well as the total runtime of the test.

Below is the summarized output of the benchmark for solutions 1-20, run on a Ryzen 5 3600

```
...
Took 0min 1s 73ms 339us for all available solutions to complete
Time in descending order:
 10.rs:    0min    0s  585ms 541us
  7.rs:    0min    0s  178ms 239us
 14.rs:    0min    0s  134ms 565us
  5.rs:    0min    0s   97ms 846us
 12.rs:    0min    0s   40ms 610us
  4.rs:    0min    0s   22ms 743us
  3.rs:    0min    0s    2ms 185us
 11.rs:    0min    0s    1ms 575us
  9.rs:    0min    0s    1ms 103us
  8.rs:    0min    0s    1ms  49us
  6.rs:    0min    0s    0ms 946us
  1.rs:    0min    0s    0ms 922us
 13.rs:    0min    0s    0ms 870us
  2.rs:    0min    0s    0ms 826us
 15.rs:    0min    0s    0ms 784us
 18.rs:    0min    0s    0ms 700us
 16.rs:    0min    0s    0ms 693us
 20.rs:    0min    0s    0ms 671us
 17.rs:    0min    0s    0ms 653us
 19.rs:    0min    0s    0ms 636us
```
