use clap::{Parser, Subcommand};
use ctrlc;

mod check;
mod gen;
mod load;

mod collider;
mod matcher;

#[macro_use]
mod utils;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "necrywagen", bin_name = "necrywagen")]
#[command(about = "Crypto key generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[clap(author, version, about, long_about = None)]
enum Commands {
    /// Generate keys
    Gen {
        /// Save keys to .tsv file (adress   segwit  privkey)
        #[clap(short, long)]
        save: bool,

        /// Path to output file
        #[clap(short, long, default_value = "./keys.tsv")]
        output: String,

        /// Number of keys
        #[clap(short, long, default_value = "1")]
        count: u32,

        /// Write head to .tsv file
        #[clap(long)]
        head: bool,

        /// Rewrite output file
        #[clap(short, long)]
        rewrite: bool,
    },

    /// Run collider
    #[command(arg_required_else_help = true)]
    Collide {
        /// Path to tsv file with btc addresses
        path: String,

        /// Number of threads
        #[clap(short, long, default_value = "1")]
        threads: u32,
    },

    /// Match str in generated keys
    Match {
        /// String to match
        match_str: String,

        /// Number of threads
        #[clap(short, long, default_value = "1")]
        threads: u32,
    },
}

fn main() {
    ctrlc::set_handler(move || {
        println!("\n");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    match args.cmd {
        Commands::Gen {
            save,
            output,
            count,
            head,
            rewrite,
        } => {
            gen::gen(save, output, count, head, rewrite);
        }
        Commands::Collide { path, threads } => {
            collider::collide(path, threads);
        }
        Commands::Match { match_str, threads } => {
            matcher::match_adresses(match_str, threads);
        }
    }

    println!();
}
