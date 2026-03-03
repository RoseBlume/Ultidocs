use std::env;
use std::path::Path;
use ultibuilder::Config;
use std::process::exit;

pub struct Cli {
    pub subcommand: Subcommand,
    pub config: Option<Config>,
}

pub enum Subcommand {
    New,
    Build,
    Dev,
    Lint(LintArgs),
    Help,
    Error,
}

#[derive(Clone)]
pub struct LintArgs {
    pub print_fixes: bool,
    pub fix: bool,
    pub verbosity: u8, // 1 = warnings only, 2 = default, 3 = info
}

impl LintArgs {
    pub fn parse(args: &[String]) -> Self {
        let mut print_fixes = false;
        let mut fix = false;
        let mut verbosity = 2u8;

        for arg in args {
            match arg.as_str() {
                "--fix" => fix = true,
                "--print" | "-p" => print_fixes = true,
                "--info" => verbosity = 3,
                "--no-warnings" | "--no-warn" => verbosity = 1,
                _ => {}
            }
        }

        Self { print_fixes, fix, verbosity }
    }
}

impl Cli {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().skip(1).collect(); // skip program name
        if args.is_empty() {
            return Self { subcommand: Subcommand::Help, config: None };
        }

        let cmd = &args[0];

        // Only these commands need a config file
        let config = match cmd.as_str() {
            "build" | "dev" | "lint" => {
                let path = Path::new("ulticonfig.json");
                let path_str = "ulticonfig.json";
                if !path.exists() {
                    eprintln!("Error: configuration file '{}' not found", path.display());
                    exit(1);
                }
                match Config::from_file(path_str) {
                    Ok(cfg) => Some(cfg),
                    Err(e) => {
                        eprintln!("Error reading config: {}", e);
                        exit(1);
                    }
                }
            }
            _ => None,
        };

        let subcommand = match cmd.as_str() {
            "new" => Subcommand::New,
            "build" => Subcommand::Build,
            "dev" => Subcommand::Dev,
            "lint" => {
                let lint_args = LintArgs::parse(&args[1..]);
                Subcommand::Lint(lint_args)
            },
            "help" => Subcommand::Help,
            _ => Subcommand::Error,
        };

        Self { subcommand, config }
    }
}