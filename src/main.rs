use clap::Parser;

/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
enum Args {
    /// Acquire a token.
    Auth,
    /// Clear a token.
    Clear,
}

fn main() {
    match Args::parse() {
        Args::Auth => println!("Acquired a token."),
        Args::Clear => println!("Cleared a token."),
    }
}
