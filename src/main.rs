#![feature(int_log)]

use clap::{ArgEnum, Parser};
use strum_macros::{Display, EnumString};

mod aks;
mod miller_rabin;

#[derive(Copy, Clone, Debug, Display, EnumString, ArgEnum)]
enum Algorithm {
    AKS,
    MillerRabin,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, arg_enum, default_value_t = Algorithm::AKS)]
    algorithm: Algorithm,
    number: u128,
}

fn main() {
    let cli = Cli::parse();
    if cli.number <= 0 {
        println!("must input natural number.");
        return;
    }
    match cli.algorithm {
        Algorithm::AKS => println!("{}", aks::is_prime(cli.number)),
        Algorithm::MillerRabin => println!("{}", miller_rabin::is_prime(cli.number)),
    }
}
