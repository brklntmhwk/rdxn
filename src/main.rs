use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input numerical value (64-bit floating point numbers acceptable)
    input: f64,
    /// Output's base
    /// Defaults to 16
    #[arg(short, long, default_value_t = 16)]
    base: usize,
}

fn main() {
    let args = Args::parse();

    let result = rdxn::fmt_radix(args.input, args.base).unwrap();

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
