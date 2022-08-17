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

fn translate(args: Args) -> Vec<String> {
    match args {
        Args::Auth(target) => {
            let mut args = vec![
                String::from("--client"),
                target.client,
                String::from("--tenant"),
                target.tenant,
                String::from("--resource"),
                String::from(" "),
            ];
            for scope in target.scopes {
                args.push(String::from("--scope"));
                args.push(scope);
            }

            args
        }
        Args::Clear(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{translate, Args, Target};
    use pretty_assertions::assert_eq;

    #[test]
    fn auth_command() {
        let args = Args::Auth(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz")],
        });
        let expected = [
            "--client",
            "foo",
            "--tenant",
            "bar",
            "--resource",
            " ",
            "--scope",
            "baz",
        ];
        let subject = translate(args);
        assert_eq!(subject, expected);
    }

    #[test]
    fn auth_command_multiple_scopes() {
        let args = Args::Auth(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz"), String::from("quux")],
        });
        let expected = [
            "--client",
            "foo",
            "--tenant",
            "bar",
            "--resource",
            " ",
            "--scope",
            "baz",
            "--scope",
            "quux",
        ];
        let subject = translate(args);
        assert_eq!(subject, expected);
    }
}
