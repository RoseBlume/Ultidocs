pub fn transform_web(src: &str, path: &str) -> String {
    #[cfg(feature = "minify_web")]
    {
        if path.ends_with(".css") {
            return ultiminify::minify_css(src);
        }
        if path.ends_with(".js") {
            return ultiminify::minify_js(src);
        }
    }

    #[cfg(feature = "format_web")]
    {
        if path.ends_with(".css") {
            return ultiminify::format_css(src);
        }
        if path.ends_with(".js") {
            return ultiminify::format_js(src);
        }
    }

    src.to_string()
}