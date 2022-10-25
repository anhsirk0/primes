use clap::Parser;

/// Prime numbers generator / checker
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct PrimesArgs {
    /// Check if a number(s) is prime.
    #[clap(short, long, value_parser, multiple = true)]
    pub check: Vec<u64>,

    /// Print primes between two numbers.
    #[clap(
        short,
        long,
        value_parser,
        multiple = true,
        min_values = 2,
        max_values = 2
    )]
    pub range: Vec<usize>,

    /// Print first n primes.
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub n: usize,
}
