use clap::Parser;

/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
struct Args;

fn main() {
    Args::parse();
}
