use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::{check, gen};

pub fn match_adresses(match_str: String, threads: Option<u32>) {
    println!("NeCryWaGen - Matcher\n-----------------\n");
    let start = Instant::now();

    let k = 0;

    let threads = threads.unwrap_or(16);
    let regex = Regex::new(&match_str).unwrap();

    println!("Threads: {}", threads);

    let k_mutex = Arc::new(Mutex::new(k));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let k_mutex = Arc::clone(&k_mutex);
            let regex = regex.clone();

            thread::spawn(move || {
                loop {
                    let keys = gen::generate_keys();

                    if check::is_match(&regex, &keys) {
                        println!("---");
                        gen::print_keys(keys.0, keys.1, keys.2);
                        println!("---")
                    }

                    // Increment k and check for the 10000th iteration

                    let mut k = k_mutex.lock().unwrap();
                    *k += 1;

                    if *k % 100000 == 0 {
                        let duration = start.elapsed();
                        print!(
                            " {} keys; {} keys/sec; {:?}\t\t",
                            k.to_formatted_string(&Locale::en),
                            *k as f64 / duration.as_secs_f64(),
                            duration
                        );
                    }
                    print!("\r");
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
