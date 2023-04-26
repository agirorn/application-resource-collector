use rand::rngs::ThreadRng;
use rand::Rng;
use std::{thread, time::Duration};

// This is slow allication that does seom number chrunching and spends cpu time
// so that it can be monitored for cpu usage.

fn main() {
    let mut rng = rand::thread_rng();
    let delay = Duration::from_millis(rng.gen::<u8>().into());
    loop {
        work(&mut rng, &delay);
    }
}

fn work(rng: &mut ThreadRng, delay: &Duration) {
    let max = rng.gen::<u8>().into();

    report(max, delay, workit(rng, max));
    thread::sleep(*delay);
}

fn workit(rng: &mut ThreadRng, max: i64) -> i64 {
    let mut result = 0;
    let mut list: Vec<i64> = vec![];
    list.push(0);
    // let should_run_inner_loop = rng.gen::<u8>() % 2 == 1;
    // let should_run_inner_most_loop = max % 2 == 1;
    let should_run_inner_loop = true;
    let should_run_inner_most_loop = true;
    for i in 0..max {
        result += i;
        list.push(i);
        if should_run_inner_loop {
            for o in 0..max {
                result -= o;
                list.push(o);
                if should_run_inner_most_loop {
                    for j in 0..max {
                        result += j;
                        list.push(j);
                    }
                }
            }
        }
    }
    result
}

fn report(max: i64, delay: &Duration, result: i64) {
    println!(
        "max: {} -- sleeping for: {} -- Result: {}",
        format!("{: >3}", max),
        format!("{: >3}", delay.as_millis()),
        format!("{: >10}", result)
    );
}
