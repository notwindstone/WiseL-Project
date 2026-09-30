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

pub fn tokenize(source: &String) -> Vec<Token> {
    let token = Token {
        kind: TokenKind::RETURN,
        value: String::from("tomorrow"),
    };
    
    vec![token]
}