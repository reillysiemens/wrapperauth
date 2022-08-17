use clap::Parser;

/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
enum Args {
    /// Acquire a token.
    Auth {
        /// Client ID.
        client: String,
        /// Tenant ID.
        tenant: String,
        /// Requested scopes.
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
