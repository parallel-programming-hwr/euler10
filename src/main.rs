use may::go;
use may::sync::mpmc::{channel, Sender};
use num_cpus;
use std::time::{Instant};

fn main() {
    may::config().set_workers(num_cpus::get());
    let (tx, rx) = channel::<u64>();
    let num_threads: u16 = num_cpus::get() as u16;
    println!("Starting {} threads", num_threads);
    for i in 0u16..num_threads {
        let tx = tx.clone();
        go!(move || {
            get_primes(1 + (2 * &i) as u64, (&num_threads * 2) as u64, 2_000_000, &tx);
        });
    }
    std::mem::drop(tx);
    let time_start = Instant::now();
    let mut prime_sum: u128 = 2;
    // receives all prime numbers via the channel receiver.
    // The received prime numbers are stored in a vector
    for prime in rx {
        prime_sum += prime as u128;
    }
    println!("Prime Sum: {}", prime_sum);
    println!("Solution took: {} ms", time_start.elapsed().as_millis())
}

/// Calculates primes and increases by incr with every iteration
/// Resulting prime numbers are sent via the tx sender.
fn get_primes(start: u64, incr: u64, stop_after: u64, tx: &Sender<u64>) {
    let mut num = start;
    while num < stop_after {
        let mut is_prime = true;
        if num == 2 {
            tx.send(num).unwrap();
            num += incr;
            continue;
        }
        if (num < 2) | (num != 2 && num % 2 == 0) {
            num += incr;
            continue;
        }
        let max = (num as f64).sqrt().ceil() as u64;
        for i in (3u64..=max).step_by(2) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            tx.send(num).unwrap();
        }
        num += incr;
    }
}
