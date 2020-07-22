use std::{thread, time::*};

const NUM_ITERS: usize = 10_000_000;
const NUM_THREADS: usize = 1;

fn do_yields() -> Vec<Duration> {
    let mut durs = Vec::with_capacity(NUM_ITERS);
    for _ in 0..NUM_ITERS {
        let yielded = Instant::now();
        thread::yield_now();
        let resume = Instant::now();
        durs.push(resume.checked_duration_since(yielded).unwrap());
    }
    durs
}

fn main() {
    let start = Instant::now();
    eprintln!(
        "testing w/{:#?} threads for {:#?} iters",
        NUM_THREADS, NUM_ITERS
    );

    let mut js = Vec::with_capacity(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        js.push(thread::spawn(do_yields));
    }

    eprintln!("threads spawned");

    let mut durs = js
        .into_iter()
        .map(thread::JoinHandle::join)
        .map(Result::unwrap)
        .flatten()
        .collect::<Vec<Duration>>();

    durs.sort_unstable();

    let min = durs[0];
    let max = durs[durs.len() - 1];
    let med = durs[durs.len() / 2];
    let avg = durs.iter().copied().sum::<Duration>() / durs.len() as u32;
    let nines = [9usize, 99, 999, 9999, 99999]
        .iter()
        .map(|nine| durs.len() * nine / (nine + 1))
        .map(|ix| durs[ix].as_micros())
        .collect::<Vec<_>>();

    eprintln!("done in {} seconds", start.elapsed().as_secs());
    eprintln!("min: {} micros", min.as_micros());
    eprintln!("max: {} micros", max.as_micros());
    eprintln!("med: {} micros", med.as_micros());
    eprintln!("avg: {} micros", avg.as_micros());
    eprintln!("90th percentile: {}", nines[0]);
    eprintln!("99th percentile: {}", nines[1]);
    eprintln!("99.9th percentile: {}", nines[2]);
    eprintln!("99.99th percentile: {}", nines[3]);
    eprintln!("99.999th percentile: {}", nines[4]);
}
