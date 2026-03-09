use crate::engine::highlight_with_classes;
use crate::langs::get_language;

pub fn highlight(code: &str, language: &str, css: &mut crate::Css) -> String {
    css.add_lang(language);
    let (keywords, symbols) = get_language(language);
    highlight_with_classes(code, keywords, symbols, language)
}