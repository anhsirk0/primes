mod args;
use args::PrimesArgs;
use clap::Parser;
use colored::*;
use primes::{PrimeSet, Sieve};

fn main() {
    let args = PrimesArgs::parse();

    let mut pset = Sieve::new();

    if args.n > 1 {
        for (_ix, p) in pset.iter().enumerate().take(args.n) {
            println!("{}", p);
        }
    }

    if args.check.len() > 0 {
        for p in args.check.iter() {
            if pset.is_prime(*p) {
                println!("{} {}", p.to_string().green(), "is a prime".green())
            } else {
                println!("{} {}", p.to_string().red(), "is not a prime".red())
            }
        }
    }

    if args.range.len() >= 2 {
        let start = args.range[0];
        let n = args.range[1];
        let (ix, _p) = pset.find(start as u64);
        for (_ix, p) in pset.iter().enumerate().skip(ix).take(n) {
            println!("{}", p);
        }
    }
}
