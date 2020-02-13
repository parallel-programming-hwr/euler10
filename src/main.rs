use may::go;
use may::sync::mpmc::{channel, Sender};
use num_cpus;
use std::env;
use std::time::{Instant, Duration};

fn main() {
    may::config().set_workers(num_cpus::get());
    let (tx, rx) = channel::<u64>();
    let num_threads: u16 = num_cpus::get() as u16;
    let mut start: u64 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        start = args[1].parse::<u64>().unwrap();
    }
    if &start % 2 == 0 {
        start += 1;
    }
    println!("Starting {} threads", num_threads);
    for i in 0u16..num_threads {
        let tx = tx.clone();
        go!(move || {
            get_primes(start + (2 * &i) as u64, (&num_threads * 2) as u64, 2_000_000, &tx);
        });
        println!("Started thread {}", i);
    }
    let time_start = Instant::now();
    let mut primes: Vec<u64> = vec![];
    primes.push(2);
    // receives all prime numbers via the channel receiver.
    // The received prime numbers are stored in a vector
    loop {
        let result = rx.recv_timeout(Duration::from_millis(10));
        match result {
            Err(_) => break,
            Ok(prime) => {
                primes.push(prime);
                println!("\r{: <30}", prime);
            }
        }
    }
    let mut prime_sum: u128 = 0;
    for prime in primes {
        prime_sum += prime as u128;
    }
    println!();
    println!("Prime Sum: {}", prime_sum);
    println!("Solution took: {} ms", time_start.elapsed().as_millis())
}

/// Calculates primes and increases by incr with every iteration
/// Resulting prime numbers are sent via the tx sender.
fn get_primes(start: u64, incr: u64, stop_after: u64, tx: &Sender<u64>) {
    println!("Hi, I'm a thread.");
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
