use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input positive integers
    input: f64,
    /// Output's base
    #[arg(short, long)]
    base: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let result = rdxn::fmt_radix(args.input, args.base).unwrap();

    println!("{} -> {}", args.input, result);

    Ok(())
}
