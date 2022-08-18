use std::process::Command;

use clap::Parser;

// Rust is able to automatically implement some traits (think interfaces) via
// the `#[derive]` attribute. Clap's Parser trait is implemented as a macro,
// which enables it to take advantage of this behavior.
//
// See also:
// - https://doc.rust-lang.org/rust-by-example/trait/derive.html
// - https://docs.rs/clap/latest/clap/trait.Parser.html
#[derive(Parser)]
struct Target {
    // Attributes like `#[clap(long)]` here are called "derive macro helper
    // attributes". They're inert on their own, but derive macro is allowed to
    // read them. In this case that changes the implementation of `Parser`.
    //
    // See also:
    // - https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes
    /// Client ID.
    #[clap(long)]
    client: String,
    /// Tenant ID.
    #[clap(long)]
    tenant: String,
    // In Rust, a `Vec`, short for "vector", is a growable array type. In some
    // ways it's similar to a `list` in Python or a `List<T>` in C#.
    //
    // See also:
    // - https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// Requested scopes.
    #[clap(long, required = true)]
    scopes: Vec<String>,
}

// Rust's macros are very powerful. They're able to transform the Rust AST
// (Abstract Syntax Tree) at compile time. In this case, having access to that
// syntax tree is how the derive macro for the `Parser` trait is able to access
// the doc comment (///) and set it as help text for the command line help.
//
// Compared with how other attribute systems work (take C# for example), macros
// are expanded at compile time, so you're less likely to run into costly
// performance pitfalls like runtime reflection on types.
//
// See also:
// - https://doc.rust-lang.org/book/ch19-06-macros.html
// - https://doc.rust-lang.org/reference/macros-by-example.html
// - https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/attributes/
//
/// Do the good auth.
#[derive(Parser)]
#[clap(version)]
enum Args {
    // Rust's enums have super powers! Also known as "Algebraic Data Types",
    // they offer a lot more functionality than enums you'd find in some other
    // languages. They aren't restricted to names for integer values and they
    // can contain other types (even anonymous structs!).
    //
    // See also:
    // - https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
    // - https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    // - https://doc.rust-lang.org/stable/std/keyword.enum.html
    // - https://patshaughnessy.net/2018/3/15/how-rust-implements-tagged-unions
    /// Acquire a token.
    Auth(Target),
    /// Clear a token.
    Clear(Target),
}

// Traits in Rust are similar to the concept of an interface in some languages,
// but generally a bit *more* powerful. Here, we implement the `From` trait, a
// very common trait in Rust. The trait is generic, so we provide a specific
// implementation which describes how get a `Vec<String>` from a `Target`.
// You'll see `From` used for transformations between types all over the place.
// In fact, the `String::from()` you see elsewhere is one such usage!
//
// See also:
// - https://doc.rust-lang.org/std/convert/trait.From.html
// - https://stackoverflow.com/questions/69477460/is-rust-trait-the-same-as-java-interface
impl From<Target> for Vec<String> {
    fn from(target: Target) -> Self {
        // Variables in Rust are immutable by default. If you want to mutate a
        // particular variable you need to annotate it with `mut`, which signals
        // to the compiler that you intend to change it. The compiler can then
        // use that extra information to make certain safety guarantees not
        // easily found in other languages. In this case we have to mark `args`
        // as mutable so that we can later call `.push()` to append new values
        // to it.
        //
        // See also:
        // - https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
        // - https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
        // - https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html
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
}

fn main() {
    // As in some other languages, variables in Rust can be "re-bound" with the
    // same name, even if the type is very different. Here `args` is an `Args`
    // enum at first, but after translation we make `args` a `Vec<String>`. If
    // it helps you can explicitly annotate these. For example,
    //
    //   let args: Args = Args::parse();
    //   let args: Vec<String> = translate(args);
    //
    // However, this usually isn't warranted. The Rust compiler is very good
    // with type inference and will tell you if you've made a mistake.
    //
    // See also:
    // - https://doc.rust-lang.org/rust-by-example/variable_bindings.html
    let args = Args::parse();
    let args = translate(args);
    // You may have seen something similar to this "method chaining" syntax used
    // to create a Command in other languages. This particular example uses the
    // "Builder Pattern", which you might also have seen in Java or C#. The
    // `.spawn()` method here is an example of a "finalizer", which consumes the
    // result of previous methods in the chain and does something with it. Not
    // all method chaining in Rust is using this pattern, but it's common.
    //
    // See also:
    // - https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
    let result = Command::new("azureauth").args(args).spawn();
    // Rust doesn't (currently) have anything that resembles exception handling.
    // Instead, in a method similar to, but (personally) more robust than Go,
    // errors are communicated "up the stack" by values. That is, if a function
    // succeeds it returns the intended value, and if it fails it returns an
    // error.
    //
    // In Rust, these values are typically some variation on the `Result<T, U>`
    // enum, which can be either `Result::Ok(T)` or `Result::Err(U)`. Callers
    // then use pattern matching to determine what to do next.
    //
    // See also:
    // - https://doc.rust-lang.org/std/result/index.html
    // - https://ruudvanasseldonk.com/2015/06/17/exceptional-results-error-handling-in-csharp-and-rust
    // - https://blog.burntsushi.net/rust-error-handling/
    // - https://blog.burntsushi.net/unwrap/
    match result {
        Ok(_) => println!("Spawned AzureAuth process."),
        Err(err) => eprintln!("Failed to spawn AzureAuth process: {err}"),
    }
}

// You may have noticed that this function has a return type, `Vec<String>`,
// but nowhere is `return` actually written. `return` is a keyword in Rust and
// it is used, but usually as a means of early return. Rust is generally an
// "expression oriented" language, and as such it allows the last expression
// (in this case what results from the `match`) to be returned without the need
// for a keyword.
//
// See also:
// - https://doc.rust-lang.org/std/keyword.return.html
fn translate(args: Args) -> Vec<String> {
    // Rust's pattern matching also has super powers! There's more to pattern
    // matching than can reasonably be covered in one small comment, so you're
    // encouraged to read more. The important thing to note here is that Rust
    // enforces exhaustiveness in pattern matching. Unlike with `case`
    // statements in some languages Rust simply won't compile if you don't
    // handle all the cases. Here we'd be prevented from only handling
    // `Args::Auth` and ignoring `Args::Clear`.
    //
    // See also:
    // - https://doc.rust-lang.org/book/ch06-02-match.html
    match args {
        // Rust's `From` trait is inherently linked with another trait called
        // `Into`, which effectively does the same thing from the opposite type.
        // If you can use `From`, you can also use `Into`.
        //
        // These are equivalent.
        //
        //   let args = Vec::from(target);
        //   let args: Vec<String> = target.into();
        //
        // You don't always need to add type annotations either, often the type
        // inference is smart enough. Which to choose is largely a matter of
        // preference. If not for adding clarity in a demo I would have written
        // `target.into()` here.
        //
        // See also:
        // - https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
        Args::Auth(target) => Vec::from(target),
        Args::Clear(target) => {
            let mut args = Vec::from(target);
            args.push(String::from("--clear"));
            // Match arms are also expressions! So this last `args` here is not
            // an accident and is actually important. It's returning `args` from
            // this match arm, which in turn returns it from the `match`
            // statement one level up.
            args
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{translate, Args, Target};
    use pretty_assertions::assert_eq;

    const EXPECTED: [&str; 8] = [
        "--client",
        "foo",
        "--tenant",
        "bar",
        "--resource",
        " ",
        "--scope",
        "baz",
    ];

    #[test]
    fn auth_command() {
        let args = Args::Auth(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz")],
        });
        let subject = translate(args);

        assert_eq!(subject, EXPECTED);
    }

    #[test]
    fn auth_command_multiple_scopes() {
        let args = Args::Auth(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz"), String::from("quux")],
        });
        let expected = [&EXPECTED[..], &["--scope", "quux"]].concat();
        let subject = translate(args);

        assert_eq!(subject, expected);
    }

    #[test]
    fn clear_command() {
        let args = Args::Clear(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz")],
        });
        let expected = [&EXPECTED[..], &["--clear"]].concat();
        let subject = translate(args);

        assert_eq!(subject, expected);
    }

    #[test]
    fn clear_command_multiple_scopes() {
        let args = Args::Clear(Target {
            client: String::from("foo"),
            tenant: String::from("bar"),
            scopes: vec![String::from("baz"), String::from("quux")],
        });
        let expected = [&EXPECTED[..], &["--scope", "quux", "--clear"]].concat();
        let subject = translate(args);

        assert_eq!(subject, expected);
    }
}
