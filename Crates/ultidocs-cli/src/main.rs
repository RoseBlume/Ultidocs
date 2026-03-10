use std::sync::{Arc};
mod helpers;
mod cli;
mod dev;
mod lint;
mod new;
use cli::{Cli, Subcommand};
use ultibuilder::{Builder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // parse args using your CLI struct

    match cli.subcommand {
        Subcommand::Build => {
            let _builder = Builder::build_fresh(&cli.config.unwrap(), true)?;
            println!("Build complete.");
            Ok(())
        }

        Subcommand::Dev => {
            // let host = "127.0.0.1";
            let host = "localhost";
            let port = 8080;
            dev::run(Arc::new(cli.config.unwrap()), host, port)
        }

        Subcommand::Lint(lint_args) => {
            // let dir = lint_args.dir;
            let do_fix = lint_args.fix;
            let print_fixes = lint_args.print_fixes;

            // Example: Verbosity hardcoded for now; you could parse it in LintArgs
            let verbose = lint_args.verbosity;

            lint::run(cli.config.unwrap(), do_fix, verbose, print_fixes)
        }

        Subcommand::New => {
            new::run()
        }

        Subcommand::Help => {
            println!("Usage:");
            println!("  ultiserver new");
            println!("  ultiserver build");
            println!("  ultiserver dev");
            println!("  ultiserver lint <dir> [--fix] [--print]");
            Ok(())
        }

        Subcommand::Error => {
            eprintln!("Error: unknown command.");
            println!("Usage:");
            println!("  ultiserver new");
            println!("  ultiserver build");
            println!("  ultiserver dev");
            println!("  ultiserver lint <dir> [--fix] [--print]");
            Ok(())
        }
    }
}