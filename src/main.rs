use clap::Parser;

#[derive(Parser)]
struct Target {
    /// Client ID.
    #[clap(long)]
    client: String,
    /// Tenant ID.
    #[clap(long)]
    tenant: String,
    /// Requested scopes.
    #[clap(long, required = true)]
    scopes: Vec<String>,
}

/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
enum Args {
    /// Acquire a token.
    Auth(Target),
    /// Clear a token.
    Clear(Target),
}

fn main() {
    match Args::parse() {
        Args::Auth(Target {
            client,
            tenant,
            scopes,
        }) => println!("Acquired a token for {client} in {tenant} with {scopes:?}."),
        Args::Clear(Target {
            client,
            tenant,
            scopes,
        }) => println!("Cleared a token for {client} in {tenant} with {scopes:?}."),
    }
}
