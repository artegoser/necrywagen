use num_format::{Locale, ToFormattedString};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::{check, gen, load, print_progress};

pub fn collide(path: String, threads: u32) {
    println!("NeCryWaGen - Collider\n");

    let adresses = load::load_adresses(&path);

    println!("Loaded {} addresses\n", adresses.len());
    println!("Threads: {}", threads);

    let k = 0;
    let start = Instant::now();

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

                    print_progress!(k_mutex, start, Locale::en);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
