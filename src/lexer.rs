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
    kind: TokenKind,
    value: String,
}

fn skip_whitespace(bytes_view: &[u8], pos: &mut usize) {
    while *pos < bytes_view.len() && (
        matches!(bytes_view[*pos], b' ' | b'\t' | b'\r')
    ) {
        // Rust does not have a postfix increment operator??
        *pos += 1;
    }
}

pub fn tokenize(source: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos: usize = 0;

    // Integer indexing into strings is not supported...
    // 'str#as_bytes' does not copy, so no overhead here.
    // '&[u8]' represents UTF-8 characters
    let bytes_view: &[u8] = source.as_bytes();

    while pos < bytes_view.len() {
        skip_whitespace(bytes_view, &mut pos);

        pos += 1;
    }

    let token = Token {
        kind: TokenKind::RETURN,
        value: String::from("tomorrow"),
    };

    tokens
}