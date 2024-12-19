use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input numerical value (64-bit floating point numbers acceptable)
    input: f64,
    /// Output's base.
    #[arg(short, long, default_value_t = 16)]
    base: usize,
    /// Decimal point(s).
    #[arg(short = 'd', long, default_value_t = 15)]
    decimal_points: usize,
}

fn main() {
    let args = Args::parse();

    let result = rdxn::fmt_radix(args.input, args.base, args.decimal_points).unwrap();

    println!("{} -> {}", args.input, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;

        Args::command().debug_assert();
    }
}
