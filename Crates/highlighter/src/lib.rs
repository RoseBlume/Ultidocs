/// Highlight code snippets and return safe HTML.
pub fn highlight(code: &str, language: &str) -> String {
    let (keywords, symbols): (&[&str], &[&str]) = match language.to_lowercase().as_str() {

        // ---------------- RUST ----------------
        "rust" | "rs" => (
            &[
                "fn","let","mut","pub","impl","struct","enum","match","use",
                "crate","mod","return","trait","where","const","static","async","await"
            ],
            &["{","}","(",")","[","]",";","::","->"],
        ),

        // ---------------- PYTHON ----------------
        "python" | "py" => (
            &[
                "def","class","import","from","return","if","elif","else",
                "for","while","try","except","True","False","None","with","as"
            ],
            &[":","(",")","[","]"],
        ),

        // ---------------- JAVASCRIPT ----------------
        "javascript" | "js" => (
            &[
                "function","let","const","var","return","if","else","class",
                "import","export","new","this","async","await","try","catch"
            ],
            &["{","}","(",")","[","]",";","=>"],
        ),

        // ---------------- TYPESCRIPT ----------------
        "typescript" | "ts" => (
            &[
                "function","let","const","var","return","if","else","class",
                "import","export","new","this","async","await",
                "interface","type","implements","public","private","readonly"
            ],
            &["{","}","(",")","[","]",";","=>",":"],
        ),

        // ---------------- HTML ----------------
        "html" => (
            &[
                "html","head","body","div","span","p","a","img",
                "script","style","title","meta","link"
            ],
            &["<",">","</","/>","="],
        ),

        // ---------------- CSS ----------------
        "css" => (
            &[
                "color","background","display","position","absolute","relative",
                "flex","grid","margin","padding","font","border"
            ],
            &["{","}",";",":",".","#"],
        ),

        // ---------------- C# ----------------
        "csharp" | "cs" | "c#" => (
            &[
                "using","namespace","class","public","private","protected",
                "static","void","int","string","bool","return","new",
                "if","else","foreach","while"
            ],
            &["{","}","(",")","[","]",";","=>"],
        ),

        // ---------------- JAVA ----------------
        "java" => (
            &[
                "class","public","private","protected","static","void",
                "int","double","boolean","new","return",
                "if","else","for","while","import","package"
            ],
            &["{","}","(",")","[","]",";"],
        ),

        // ---------------- PHP ----------------
        "php" => (
            &[
                "function","class","public","private","protected",
                "echo","return","if","else","foreach","while",
                "namespace","use","new"
            ],
            &["{","}","(",")","[","]",";","=>","$"],
        ),

        // ---------------- DART ----------------
        "dart" => (
            &[
                "class","void","int","double","String","bool",
                "return","if","else","for","while",
                "import","new","final","const"
            ],
            &["{","}","(",")","[","]",";","=>"],
        ),

        // ---------------- C ----------------
        "c" => (
            &[
                "int","char","float","double","void","return",
                "if","else","for","while","struct","typedef",
                "include"
            ],
            &["{","}","(",")","[","]",";","*","&","#"],
        ),

        // ---------------- C++ ----------------
        "cpp" | "c++" => (
            &[
                "int","char","float","double","void","return",
                "if","else","for","while","class","struct",
                "public","private","protected","namespace",
                "using","new","delete","auto","template"
            ],
            &["{","}","(",")","[","]",";","::","*","&"],
        ),

        // ---------------- NASM ----------------
        "nasm" | "asm" => (
            &[
                "mov","add","sub","jmp","cmp","push","pop",
                "call","ret","section","global"
            ],
            &[":",",","[","]"],
        ),

        // ---------------- R ----------------
        "r" => (
            &[
                "function","if","else","for","while",
                "TRUE","FALSE","NULL","library","return"
            ],
            &["{","}","(",")","[","]","<-"],
        ),

        // ---------------- SQL ----------------
        "sql" => (
            &[
                "SELECT","FROM","WHERE","INSERT","INTO","UPDATE",
                "DELETE","JOIN","LEFT","RIGHT","INNER","OUTER",
                "CREATE","TABLE","DROP","ALTER","AND","OR","NOT"
            ],
            &[",",";","(",")","*","="],
        ),
        "shell" | "bash" | "sh" => (
            &[
                "if", "then", "else", "elif", "fi",
                "for", "while", "do", "done",
                "function", "in", "case", "esac",
                "return", "break", "continue", "exit",
                "export", "readonly", "local"
            ],
            &["|", "&", "&&", "||", "!", ";", "{", "}", "(", ")", "[", "]", "<", ">", "="],
        ),

        // ---------------- WINDOWS BATCH / CMD ----------------
        "bat" | "batch" | "cmd" => (
            &[
                "echo", "set", "if", "else", "for", "in", "do", "goto",
                "call", "exit", "rem", "pause", "shift", "title", "cd",
                "mkdir", "rmdir", "copy", "move", "del", "type"
            ],
            &["<", ">", ">>", "|", "&", "&&", "||", "%", "(", ")", "!"],
        ),

        _ => (&[], &[]),
    };

    highlight_with_classes(code, keywords, symbols, language)
}

fn highlight_with_classes(
    code: &str,
    keywords: &[&str],
    symbols: &[&str],
    language: &str,
) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut in_style_block = false;
    let mut in_script_block = false;

    for line in code.lines() {
        let trimmed = line.trim_start();

        // Detect start/end of style/script blocks for HTML
        if language == "html" {
            if trimmed.starts_with("<style") {
                in_style_block = true;
            }
            if trimmed.starts_with("<script") {
                in_script_block = true;
            }

            if in_style_block || in_script_block {
                result.push_str(&escape_html(line));
                result.push('\n');

                if trimmed.ends_with("</style>") {
                    in_style_block = false;
                }
                if trimmed.ends_with("</script>") {
                    in_script_block = false;
                }
                continue; // skip normal highlighting inside these blocks
            }
        }

        let mut token = String::new();
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            // Handle string literals
            if in_string {
                token.push(ch);
                if ch == string_char {
                    result.push_str(&format!(
                        r#"<span class="string">{}</span>"#,
                        escape_html(&token)
                    ));
                    token.clear();
                    in_string = false;
                }
                continue;
            }

            // Start of string
            if ch == '"' || ch == '\'' {
                flush_token_class(&mut result, &mut token, keywords);
                in_string = true;
                string_char = ch;
                token.push(ch);
                continue;
            }

            // Logical operators &&, ||, !
            if (ch == '&' && chars.peek() == Some(&'&'))
                || (ch == '|' && chars.peek() == Some(&'|'))
            {
                flush_token_class(&mut result, &mut token, keywords);
                let op = format!("{}{}", ch, chars.next().unwrap());
                result.push_str(&format!(r#"<span class="logical">{}</span>"#, op));
                continue;
            }

            if ch == '!' {
                flush_token_class(&mut result, &mut token, keywords);
                result.push_str(r#"<span class="logical">!</span>"#);
                continue;
            }

            // Symbols
            let ch_str = ch.to_string();
            if symbols.contains(&ch_str.as_str()) {
                flush_token_class(&mut result, &mut token, keywords);
                result.push_str(&format!(r#"<span class="symbol">{}</span>"#, escape_html(&ch_str)));
                continue;
            }

            // Build alphanumeric token
            if ch.is_alphanumeric() || ch == '_' || ch == '#' {
                token.push(ch);
            } else {
                flush_token_class(&mut result, &mut token, keywords);
                result.push(ch);
            }
        }

        flush_token_class(&mut result, &mut token, keywords);
        result.push('\n');
    }

    format!(
        r#"<pre class="highlight lang-{}"><code>{}</code></pre>"#,
        language, result
    )
}

/// Flush current token and wrap in keyword class if needed
fn flush_token_class(output: &mut String, token: &mut String, keywords: &[&str]) {
    if token.is_empty() {
        return;
    }

    if keywords.contains(&token.as_str()) {
        output.push_str(&format!(r#"<span class="keyword">{}</span>"#, escape_html(token)));
    } else {
        output.push_str(&escape_html(token));
    }

    token.clear();
}

/// Escape text for HTML (preserves & in logical operators)
fn escape_html(input: &str) -> String {
    input.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}