use num_format::{Locale, ToFormattedString};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::{check, gen, load};

pub fn collide(path: String, threads: Option<u32>) {
    println!("NeCryWaGen - Collider\n");

    let adresses = load::load_adresses(&path);

    println!("Loaded {} addresses\n", adresses.len());

    let start = Instant::now();

    let k = 0;

    let threads = threads.unwrap_or(16);

    println!("Threads: {}", threads);

    let adresses_mutex = Arc::new(Mutex::new(adresses));
    let k_mutex = Arc::new(Mutex::new(k));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let adresses_mutex = Arc::clone(&adresses_mutex);
            let k_mutex = Arc::clone(&k_mutex);

            thread::spawn(move || {
                loop {
                    let keys = gen::generate_keys();

                    if check::is_intersect(&adresses_mutex.lock().unwrap(), &keys) {
                        println!("\nFOUND\n\n{:?}\n\n-----------------\n", keys);

                        // TODO: if found then transfer all btc from that address to my wallet
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
