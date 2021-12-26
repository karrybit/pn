#![feature(int_log)]

use clap::{ArgEnum, Parser};

mod aks;
mod miller_rabin;

#[derive(Copy, Clone, Debug, ArgEnum)]
enum Algorithm {
    Aks,
    MillerRabin,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, arg_enum, default_value_t = Algorithm::Aks)]
    algorithm: Algorithm,
    number: u128,
}

fn main() {
    let cli = Cli::parse();
    println!(
        "{}",
        match cli.algorithm {
            Algorithm::Aks => aks::is_prime(cli.number),
            Algorithm::MillerRabin => miller_rabin::is_prime(cli.number),
        }
    );
}
