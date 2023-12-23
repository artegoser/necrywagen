use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::{check, gen, print_progress};

pub fn match_adresses(match_str: String, threads: u32) {
    println!("NeCryWaGen - Matcher\n-----------------\n");
    println!("Threads: {}", threads);

    let k = 0;
    let regex = Regex::new(&match_str).unwrap();

    let start = Instant::now();

    let k_mutex = Arc::new(Mutex::new(k));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let k_mutex = Arc::clone(&k_mutex);
            let regex = regex.clone();

            thread::spawn(move || loop {
                let keys = gen::generate_keys();

                if check::is_match(&regex, &keys) {
                    println!("---");
                    gen::print_keys(keys.0, keys.1, keys.2);
                }

                print_progress!(k_mutex, start, Locale::fr);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
