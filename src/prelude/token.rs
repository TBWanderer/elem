#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Name(String),
    String(String),
    LeftParen,
    RightParen,
}

pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_string = String::new();
    let mut symbol_start: Option<usize> = None;
    let mut is_string = false;
    let mut is_escaped = false;

    for (byte_idx, c) in line.char_indices() {
        if is_string {
            handle_string_token(
                c,
                &mut current_string,
                &mut is_escaped,
                &mut is_string,
                &mut tokens,
            );
        } else {
            handle_normal_char(
                c,
                byte_idx,
                &mut symbol_start,
                &mut is_string,
                &mut current_string,
                &mut tokens,
                line,
            );
        }
    }

    // Handle remaining symbols after loop
    if let Some(start) = symbol_start {
        tokens.push(Token::Name(line[start..].to_string()));
    }

    // Handle unterminated string
    if is_string {
        tokens.push(Token::String(current_string));
    }

    tokens
}

fn handle_normal_char(
    c: char,
    byte_idx: usize,
    symbol_start: &mut Option<usize>,
    is_string: &mut bool,
    current_string: &mut String,
    tokens: &mut Vec<Token>,
    line: &str,
) {
    match c {
        '"' => {
            if let Some(start) = symbol_start.take() {
                tokens.push(Token::Name(line[start..byte_idx].to_string()));
            }
            current_string.push(c);
            *is_string = true;
        }
        ' ' | '.' => handle_delimiter(byte_idx, symbol_start, tokens, line),
        '(' | ')' => {
            handle_delimiter(byte_idx, symbol_start, tokens, line);
            tokens.push(if c == '(' {
                Token::LeftParen
            } else {
                Token::RightParen
            });
        }
        ';' => {
            handle_delimiter(byte_idx, symbol_start, tokens, line);
            symbol_start.take(); // Stop processing after semicolon
        }
        _ => {
            if symbol_start.is_none() {
                *symbol_start = Some(byte_idx);
            }
        }
    }
}

fn handle_string_token(
    c: char,
    current_string: &mut String,
    is_escaped: &mut bool,
    is_string: &mut bool,
    tokens: &mut Vec<Token>,
) {
    if *is_escaped {
        match c {
            'n' => current_string.push('\n'),
            't' => current_string.push('\t'),
            'r' => current_string.push('\r'),
            '\\' => current_string.push('\\'),
            '"' => current_string.push('"'),
            _ => {
                current_string.push('\\');
                current_string.push(c);
            }
        }
        *is_escaped = false;
    } else if c == '\\' {
        *is_escaped = true;
    } else if c == '"' {
        current_string.push(c);
        tokens.push(Token::String(std::mem::take(current_string)));
        *is_string = false;
    } else {
        current_string.push(c);
    }
}

fn handle_delimiter(
    byte_idx: usize,
    symbol_start: &mut Option<usize>,
    tokens: &mut Vec<Token>,
    line: &str,
) {
    if let Some(start) = symbol_start.take() {
        tokens.push(Token::Name(line[start..byte_idx].to_string()));
    }
}
