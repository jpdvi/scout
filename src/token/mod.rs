pub type TokenType<'a> = &'a str;

pub static LEFTBRACKET: &str = "{";
pub static RIGHTBRACKET: &str = "}";
pub static EOF: &str = "EOF";
pub static ILLEGAL: &str = "ILLEGAL";
pub static TEXT: &str = "TEXT";

pub struct Token<'a> {
    pub literal: String,
    pub _type: TokenType<'a>,
}

impl<'a> Token<'a> {
    pub fn new(_type : TokenType, ch : Option<char>) -> Token {
        let mut _literal = match ch {
            None => ' '.to_string(),
            _=> ch.unwrap().to_string()
        };
        Token {
            literal : _literal,
            _type : _type,
        }
    }
}

pub struct Pattern<'a> {
    pub left: Token<'a>,
    pub right: Token<'a>,
}
