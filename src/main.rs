use num_format::{Locale, ToFormattedString};
use std::time::Instant;

mod check;
mod gen;
mod load;

fn main() {
    let adresses = load::load_adresses("C:/btc/info/blockchain_bitcoin_addresses_and_balance.tsv");

    println!("Loaded {} addresses\n", adresses.len());

    println!("NeCryWaGen - btc collision experiment\n-----------------\n");
    let start = Instant::now();

    let mut k = 0;

    loop {
        let keys = gen::generate_keys();

        if check::is_intersect(&adresses, &keys) {
            println!("\nFOUND\n\n{:?}\n\n-----------------\n", keys);
        }

        k += 1;

        if k % 10000 == 0 {
            let duration = start.elapsed();
            print!(
                " {} keys; {} keys/sec; {:?}",
                k.to_formatted_string(&Locale::en),
                k as f64 / duration.as_secs_f64(),
                duration
            );
        }
        print!("\r");
    }
}
