use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub fn parse(input: &str) -> Result<JsonValue, String> {
    let mut chars = input.chars().peekable();
    parse_value(&mut chars)
}

fn parse_value<I>(chars: &mut std::iter::Peekable<I>) -> Result<JsonValue, String>
where
    I: Iterator<Item = char>,
{
    skip_ws(chars);

    match chars.peek() {
        Some('"') => parse_string(chars).map(JsonValue::String),
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('t') => {
            consume(chars, "true")?;
            Ok(JsonValue::Bool(true))
        }
        Some('f') => {
            consume(chars, "false")?;
            Ok(JsonValue::Bool(false))
        }
        Some('n') => {
            consume(chars, "null")?;
            Ok(JsonValue::Null)
        }
        Some(c) if c.is_digit(10) || *c == '-' => parse_number(chars),
        _ => Err("Unexpected character in JSON".into()),
    }
}

fn parse_object<I>(chars: &mut std::iter::Peekable<I>) -> Result<JsonValue, String>
where
    I: Iterator<Item = char>,
{
    chars.next(); // {

    let mut map = HashMap::new();

    loop {
        skip_ws(chars);

        if let Some('}') = chars.peek() {
            chars.next();
            break;
        }

        let key = parse_string(chars)?;
        skip_ws(chars);

        if chars.next() != Some(':') {
            return Err("Expected ':'".into());
        }

        let value = parse_value(chars)?;
        map.insert(key, value);

        skip_ws(chars);

        match chars.peek() {
            Some(',') => {
                chars.next();
            }
            Some('}') => continue,
            _ => return Err("Expected ',' or '}'".into()),
        }
    }

    Ok(JsonValue::Object(map))
}

fn parse_array<I>(chars: &mut std::iter::Peekable<I>) -> Result<JsonValue, String>
where
    I: Iterator<Item = char>,
{
    chars.next(); // [

    let mut arr = Vec::new();

    loop {
        skip_ws(chars);

        if let Some(']') = chars.peek() {
            chars.next();
            break;
        }

        arr.push(parse_value(chars)?);

        skip_ws(chars);

        match chars.peek() {
            Some(',') => { chars.next(); }
            Some(']') => continue,
            _ => return Err("Expected ',' or ']'".into()),
        }
    }

    Ok(JsonValue::Array(arr))
}

fn parse_string<I>(chars: &mut std::iter::Peekable<I>) -> Result<String, String>
where
    I: Iterator<Item = char>,
{
    chars.next(); // "

    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c == '"' {
            return Ok(result);
        }
        result.push(c);
    }

    Err("Unterminated string".into())
}

fn parse_number<I>(chars: &mut std::iter::Peekable<I>) -> Result<JsonValue, String>
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();

    while let Some(c) = chars.peek() {
        if c.is_digit(10) || *c == '.' || *c == '-' {
            num.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    let parsed = num.parse::<f64>()
        .map_err(|_| "Invalid number")?;

    Ok(JsonValue::Number(parsed))
}

fn skip_ws<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    loop {
        // Skip whitespace
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }

        // Check for comment start
        if chars.peek() == Some(&'/') {
            chars.next(); // consume '/'

            match chars.peek() {
                Some('/') => {
                    // Single-line comment
                    chars.next(); // consume second '/'
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some('*') => {
                    // Multi-line comment
                    chars.next(); // consume '*'
                    loop {
                        match chars.next() {
                            Some('*') => {
                                if chars.peek() == Some(&'/') {
                                    chars.next(); // consume '/'
                                    break;
                                }
                            }
                            Some(_) => {}
                            None => break, // Unterminated comment
                        }
                    }
                }
                _ => {
                    // It was just a single '/', not a comment.
                    // Put it back logically by returning — parser will error correctly.
                    return;
                }
            }
        } else {
            break;
        }
    }
}

fn consume<I>(chars: &mut std::iter::Peekable<I>, expected: &str) -> Result<(), String>
where
    I: Iterator<Item = char>,
{
    for ec in expected.chars() {
        if chars.next() != Some(ec) {
            return Err("Invalid token".into());
        }
    }
    Ok(())
}