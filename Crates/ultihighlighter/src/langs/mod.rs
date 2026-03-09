pub mod rust;
pub mod python;
pub mod javascript;
pub mod typescript;
pub mod html;
pub mod css;
pub mod csharp;
pub mod java;
pub mod php;
pub mod dart;
pub mod c;
pub mod cpp;
pub mod nasm;
pub mod r;
pub mod sql;
pub mod shell;
pub mod batch;
pub mod consts;
pub fn get_language(lang: &str) -> (&'static [&'static str], &'static [&'static str]) {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => rust::LANG,
        "python" | "py" => python::LANG,
        "javascript" | "js" => javascript::LANG,
        "typescript" | "ts" => typescript::LANG,

        "html" => html::LANG,
        "css" => css::LANG,

        "csharp" | "cs" | "c#" => csharp::LANG,
        "java" => java::LANG,
        "php" => php::LANG,
        "dart" => dart::LANG,

        "c" => c::LANG,
        "cpp" | "c++" => cpp::LANG,

        "nasm" | "asm" => nasm::LANG,
        "r" => r::LANG,
        "sql" => sql::LANG,

        "shell" | "bash" | "sh" => shell::LANG,
        "bat" | "batch" | "cmd" => batch::LANG,

        _ => (&[], &[]),
    }
}
