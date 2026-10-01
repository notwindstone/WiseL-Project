#[derive(Clone, Copy)]
pub enum TokenKind {
    FORMAT,
    STRING,

    FUNC,
    ASM,
    IDENT,

    LET,
    MUT,
    COLON,
    ASSIGN,
    STATIC,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    COMMA,
    NUMBER,

    DIRECTIVE,
    NEWLINE,

    WHILE,
    IF,
    ELSE,
    BREAK,

    EQ,
    PLUSPLUS,
    DOT,

    USELIB,

    DIV,
    MOD,

    ARROW,

    RETURN,
    END
}
pub struct Token {
    // 'type' is a reserved keyword lol
    kind : TokenKind,
    value: String,
}

pub struct OpToken {
    kind: TokenKind,
    text: &'static str,
}

// Skips spaces, tabs, and carriage returns
fn skip_whitespace(bytes_view: &[u8], pos: &mut usize) {
    // Rust does not have a postfix increment operator??
    while *pos < bytes_view.len() && matches!(bytes_view[*pos], b' ' | b'\t' | b'\r') { *pos += 1; }
}

// Reads string literals including quotes and properly handles escaped quotes
fn read_string(source: &str, bytes_view: &[u8], pos: &mut usize) -> Result<String, String> {
    let start: usize = *pos;
    // Capture the used quote symbol (might be either ' or ")
    let quote: u8 = bytes_view[start];

    let mut escaped: bool = false;

    // We are borrowing a slice without any memory re-allocations
    for current_byte in &bytes_view[start..] {
        // 'start' is at the opening quote index, so skip that quote
        if *pos == start {
            *pos += 1;
            continue;
        }

        *pos += 1;

        if escaped {
            escaped = false;
            continue;
        }

        match *current_byte {
            _byte if _byte == quote => return Ok(source[start..*pos].to_owned()),
            b'\\' => escaped = true,
            _     => {},
        }
    }

    Err(format!("No closing quote was found for {}", &source[start..]))
}

// Reads identifiers, allowing letters, digits, and underscores
fn read_identifier(source: &str, bytes_view: &[u8], pos: &mut usize) -> String {
    let start: usize = *pos;

    while *pos < bytes_view.len() && (
        bytes_view[*pos].is_ascii_alphanumeric() ||
        bytes_view[*pos] == b'_'
    ) { *pos += 1; }

    source[start..*pos].to_owned()
}

// Maps the keyword string to a token kind
fn get_keyword_type(word: &str) -> TokenKind {
    match word {
        "func"   => TokenKind::FUNC,
        "if"     => TokenKind::IF,
        "else"   => TokenKind::ELSE,
        "while"  => TokenKind::WHILE,
        "break"  => TokenKind::BREAK,
        "let"    => TokenKind::LET,
        "mut"    => TokenKind::MUT,
        "asm"    => TokenKind::ASM,
        "static" => TokenKind::STATIC,
        "return" => TokenKind::RETURN,
        "Format" => TokenKind::FORMAT,
        "UseLib" => TokenKind::USELIB,
        _        => TokenKind::IDENT,
    }
}

static OP_TABLE: &[OpToken] = &[
    OpToken { text: "->", kind: TokenKind::ARROW },
    OpToken { text: "==", kind: TokenKind::EQ },
    OpToken { text: "++", kind: TokenKind::PLUSPLUS },
    OpToken { text: "<=", kind: TokenKind::IDENT },
    OpToken { text: ">=", kind: TokenKind::IDENT },
    OpToken { text: "!=", kind: TokenKind::IDENT },
    OpToken { text: "=",  kind: TokenKind::ASSIGN },
    OpToken { text: "+",  kind: TokenKind::IDENT },
    OpToken { text: "-",  kind: TokenKind::IDENT },
    OpToken { text: "*",  kind: TokenKind::IDENT },
    OpToken { text: "<",  kind: TokenKind::IDENT },
    OpToken { text: ">",  kind: TokenKind::IDENT },
    OpToken { text: "!",  kind: TokenKind::IDENT },
    OpToken { text: "/",  kind: TokenKind::DIV },
    OpToken { text: "%",  kind: TokenKind::MOD },
];

// Extracts the operator token if there is any
fn extract_op_token(bytes_view: &[u8], pos: &mut usize) -> Option<&'static OpToken> {
    OP_TABLE
        .iter()
        .find(|op| bytes_view[*pos..].starts_with(op.text.as_bytes()))
}

fn read_number(source: &str, bytes_view: &[u8], pos: &mut usize) -> String {
    let start: usize = *pos;

    while *pos < bytes_view.len() && bytes_view[*pos].is_ascii_digit() { *pos += 1; }
    if *pos < bytes_view.len() && matches!(bytes_view[*pos], b'h' | b'H') { *pos += 1 }

    source[start..*pos].to_owned()
}

fn get_token_kind(byte: u8) -> Option<TokenKind> {
    Some(match byte {
        b'(' => TokenKind::LPAREN,
        b')' => TokenKind::RPAREN,

        b'{' => TokenKind::LBRACE,
        b'}' => TokenKind::RBRACE,

        b'[' => TokenKind::LBRACKET,
        b']' => TokenKind::RBRACKET,

        b','  => TokenKind::COMMA,
        b'\n' => TokenKind::NEWLINE,
        b'"'  => TokenKind::STRING,
        b'\'' => TokenKind::STRING,

        // Complex kinds are determined at a latter point ('@', '.', ':')
        _ => return None,
    })
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos: usize = 0;

    // Integer indexing into strings is not supported...
    // 'str#as_bytes' does not copy, so no overhead here.
    // Also, 'u8' can represent ASCII characters but not Unicode characters
    let bytes_view: &[u8] = source.as_bytes();

    while pos < bytes_view.len() {
        skip_whitespace(bytes_view, &mut pos);
        if pos >= bytes_view.len() { break; }

        let current_byte: u8 = bytes_view[pos];

        if current_byte == b'/' && pos + 1 < bytes_view.len() && bytes_view[pos + 1] == b'/' {
            while pos < bytes_view.len() && bytes_view[pos] != b'\n' { pos += 1; }
            continue;
        }

        if let Some(op) = extract_op_token(bytes_view, &mut pos) {
            tokens.push(Token { kind: op.kind, value: op.text.to_owned() });
            pos += op.text.len();
            continue;
        }

        if let Some(token_kind) = get_token_kind(current_byte) {
            let token_value = if matches!(token_kind, TokenKind::STRING) {
                match read_string(source, bytes_view, &mut pos) {
                    Ok(content) => content,
                    Err(error)  => {
                        eprintln!("[ERROR] Cannot tokenize the string: {error}");
                        std::process::exit(1);
                    }
                }
            } else {
                pos += 1;
                char::from(current_byte).to_string()
            };

            tokens.push(Token { kind: token_kind, value: token_value });
            continue;
        }

        // If we got to this branch, then the current byte
        // can be '@', '.', ':', and something unknown
        match current_byte {
            b'@' => {
                pos += 1;

                let token = if pos < bytes_view.len() && bytes_view[pos].is_ascii_alphabetic() {
                    let word = read_identifier(source, bytes_view, &mut pos);
                    let start = pos;

                    while pos < bytes_view.len() &&
                        matches!(bytes_view[pos], b' ' | b'\t' | b'\n' | b'\r' | b'{') != true {
                        pos += 1;
                    }

                    Token {
                        kind : TokenKind::DIRECTIVE,
                        value: format!("{word}{}", &source[start..pos]),
                    }
                } else {
                    Token { kind : TokenKind::IDENT, value: char::from(current_byte).to_string() }
                };

                tokens.push(token);
            },
            b'.' => {
                pos += 1;

                let token = if pos < bytes_view.len() && bytes_view[pos].is_ascii_digit() {
                    Token {
                        kind : TokenKind::NUMBER,
                        value: format!(".{}", read_number(source, bytes_view, &mut pos)),
                    }
                } else if pos < bytes_view.len() && (
                    bytes_view[pos].is_ascii_alphabetic() || bytes_view[pos] == b'_'
                ) {
                    Token {
                        kind : TokenKind::IDENT,
                        value: format!(".{}", read_identifier(source, bytes_view, &mut pos)),
                    }
                } else {
                    Token { kind: TokenKind::DOT, value: String::from(".") }
                };

                tokens.push(token);
            },
            b':' => {
                pos += 1;

                let token = if pos < bytes_view.len() && (
                    bytes_view[pos].is_ascii_alphabetic() || bytes_view[pos] == b'_'
                ) {
                    Token {
                        kind : TokenKind::IDENT,
                        value: format!(":{}", read_identifier(source, bytes_view, &mut pos)),
                    }
                } else {
                    Token { kind: TokenKind::COLON, value: String::from(":") }
                };

                tokens.push(token);
            },

            _byte if _byte.is_ascii_digit()      => {
                tokens.push(Token {
                    kind : TokenKind::NUMBER,
                    value: read_number(source, bytes_view, &mut pos),
                });
            },
            _byte if _byte.is_ascii_alphabetic() || _byte == b'_' => {
                let word = read_identifier(source, bytes_view, &mut pos);
                let token_kind = get_keyword_type(&word);

                tokens.push(Token { kind: token_kind, value: word });
            },

            _ => {
                tokens.push(Token {
                    kind : TokenKind::IDENT,
                    value: char::from(current_byte).to_string(),
                });
                pos += 1;
            },
        }
    }

    tokens.push(Token { kind: TokenKind::END, value: String::from("") });
    tokens
}