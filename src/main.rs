use clap::Parser;

/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
enum Args {
    /// Acquire a token.
    Auth {
        /// Client ID.
        #[clap(long)]
        client: String,
        /// Tenant ID.
        #[clap(long)]
        tenant: String,
        /// Requested scopes.
        #[clap(long, required = true)]
        scopes: Vec<String>,
    },
    /// Clear a token.
    Clear,
}

fn main() {
    match Args::parse() {
        Args::Auth {
            client,
            tenant,
            scopes,
        } => println!("Acquired a token for {client} in {tenant} with {scopes:?}."),
        Args::Clear => println!("Cleared a token."),
    }
}
