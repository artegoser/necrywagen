use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_adresses(path: &str) -> HashSet<String> {
    let file = File::open(Path::new(path)).unwrap();
    let reader = io::BufReader::new(file);

    let mut set = HashSet::new();
    let mut k = 0;
    let n = 1000000;

    println!("-----------------\nLoading addresses...\n");

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if k % n == 0 {
                    print!(" {} m", k / n);
                }

                let address = l.split('\t').next().unwrap();
                set.insert(address.to_string());

                k += 1;
            }
            Err(_) => {}
        }

        print!("\r");
    }

    println!("");
    println!("Done!\n-----------------\n");

    set
}
