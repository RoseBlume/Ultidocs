mod linters;
use ultibuilder::Config;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::helpers::collect_files_recursive;
use ultilinter::{LintConfig, Severity, Linter, LintReport, apply_fixes_to_string};
use crate::lint::linters::md;
pub fn run(config: Config, do_fix: bool, verbose: u8, print_fixed: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut files_to_lint = Vec::new();

    if !config.content_dir.is_empty() {
        files_to_lint.extend(collect_files_recursive(Path::new(&config.content_dir)));
    }

    // for opt_file in [
    //     &config.custom_css,
    // ] {
    //     if let Some(f) = opt_file {
    //         files_to_lint.push(PathBuf::from(f));
    //     }
    // }

    // Lint each file
    for file_path in files_to_lint {
        if let Err(e) = lint_file(&file_path, do_fix, print_fixed, verbose) {
            eprintln!("Error linting {}: {}", file_path.display(), e);
        }
    }
    Ok(())
}

/// Lint a single file using ultilint
pub fn lint_file(path: &Path, do_fix: bool, print_fixed: bool, verbose: u8) -> io::Result<()> {
    let source = fs::read_to_string(path)?;

    // Auto-detect language
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let linter: Linter = match ext.as_str() {
        "md" | "markdown" => md::linter(),
        // "rs" => ultilinter::linters::rust::linter(),
        // "c" | "h" => ultilinter::linters::c::linter(),
        // "cpp" | "hpp" => ultilinter::linters::cpp::linter(),
        "html" | "htm" => ultilinter::linters::html::linter(),
        "js" => ultilinter::linters::javascript::linter(),
        // "ts" => ultilinter::linters::typescript::linter(),
        "css" => ultilinter::linters::css::linter(),
        // "php" => ultilinter::linters::php::linter(),
        // "py" => ultilinter::linters::python::linter(),
        // "dart" => ultilinter::linters::dart::linter(),
        // "sql" => ultilinter::linters::sql::linter(),
        // "cs" => ultilinter::linters::csharp::linter(),
        // "asm" | "nasm" => ultilinter::linters::nasm::linter(),
        // "r" => ultilinter::linters::r::linter(),
        _ => return Ok(()), // skip unknown extensions
    };

    let config = LintConfig::new();
    let report = linter.run(Some(path), &source, &config);

    // Filter by severity
    let filtered: Vec<_> = report.errors.into_iter().filter(|e| match e.severity {
        Severity::Error => verbose >= 1,
        Severity::Warning => verbose >= 2,
        Severity::Info => verbose >= 3,
    }).collect();

    if filtered.is_empty() && !do_fix {
        return Ok(());
    }

    // Print lint issues
    if !do_fix {
        for err in &filtered {
            println!(
                "{}:{}:{} [{}] {}",
                err.file.as_ref().map(|f: &PathBuf| f.display().to_string()).unwrap_or_default(),
                err.line,
                err.column,
                err.rule_id,
                err.message
            );
            if let Some(s) = &err.suggestion {
                println!("   Suggestion: {}", s);
            }
        }
        return Ok(());
    }

    // Apply fixes
    let fixed = apply_fixes_to_string(&LintReport { errors: filtered.clone() }, &source);

    // Overwrite original file
    fs::write(path, &fixed)?;

    // Optionally print to terminal
    if print_fixed {
        println!("--- Fixed Output ({}) ---\n{}", path.display(), fixed);
    }

    Ok(())
}