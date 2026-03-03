use std::env;
use std::fs;
use std::path::Path;

use ultilinter::{LintConfig, LintReport, Severity, Linter, apply_fixes_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [--fix] [--info] [--no-warnings] [--output <file>] [-p]", args[0]);
        return;
    }

    let file_path = &args[1];
    let path = Path::new(file_path);
    let source = fs::read_to_string(path).expect("Failed to read file");

    // Defaults
    let mut verbose: u8 = 2;
    let mut do_fix = false;
    let mut output_path: Option<&str> = None;
    let mut print_fixed = false;

    // Parse flags
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--fix" => do_fix = true,
            "--info" => verbose = 3,
            "--no-warnings" | "--no-warn" => verbose = 1,
            "--output" => {
                if i + 1 < args.len() {
                    output_path = Some(&args[i + 1]);
                    i += 1;
                } else {
                    eprintln!("--output requires a file path");
                    return;
                }
            }
            "-p" | "--print" => print_fixed = true,
            _ => {}
        }
        i += 1;
    }

    // Auto-detect language
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let linter: Linter = match ext.as_str() {
        "md" | "markdown" => ultilinter::linters::md::linter(),
        "rs" => ultilinter::linters::rust::linter(),
        "c" | "h" => ultilinter::linters::c::linter(),
        "cpp" | "hpp" => ultilinter::linters::cpp::linter(),
        "html" | "htm" => ultilinter::linters::html::linter(),
        "js" => ultilinter::linters::javascript::linter(),
        "ts" => ultilinter::linters::typescript::linter(),
        "css" => ultilinter::linters::css::linter(),
        "php" => ultilinter::linters::php::linter(),
        "py" => ultilinter::linters::python::linter(),
        "dart" => ultilinter::linters::dart::linter(),
        "sql" => ultilinter::linters::sql::linter(),
        "cs" => ultilinter::linters::csharp::linter(),
        "asm" | "nasm" => ultilinter::linters::nasm::linter(),
        "r" => ultilinter::linters::r::linter(),
        _ => {
            eprintln!("Unknown file extension: {}", ext);
            return;
        }
    };

    // Run linter
    let config = LintConfig::new();
    let report: LintReport = linter.run(Some(path), &source, &config);

    // Filter by severity
    let filtered: Vec<_> = report.errors.into_iter().filter(|e| match e.severity {
        Severity::Error => verbose >= 1,
        Severity::Warning => verbose >= 2,
        Severity::Info => verbose >= 3,
    }).collect();

    if !do_fix {
        // Print lint issues only
        if filtered.is_empty() {
            println!("No lint issues found");
            return;
        } else {
            for err in &filtered {
                println!(
                    "{}:{}:{} [{}] {}",
                    err.file.as_ref().map(|f: &std::path::PathBuf| f.display().to_string()).unwrap_or_default(),
                    err.line,
                    err.column,
                    err.rule_id,
                    err.message
                );
                if let Some(s) = &err.suggestion {
                    println!("   Suggestion: {}", s);
                }
            }
        }
        return;
    }

    // Apply fixes
    let fixed = apply_fixes_to_string(&LintReport { errors: filtered.clone() }, &source);

    // Determine where to write fixed file
    let write_path = output_path.unwrap_or(file_path);
    fs::write(write_path, &fixed).expect("Failed to write fixed file");

    // Inform user
    if output_path.is_some() {
        println!("Fixed file written to {}", write_path);
    } else {
        println!("Original file overwritten with fixed content: {}", write_path);
    }

    // Print fixed file if -p / --print is used
    if print_fixed {
        println!("--- Fixed Output ---\n{}", fixed);
    }
}