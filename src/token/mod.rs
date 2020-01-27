pub type TokenType<'a> = &'a str;

pub static LEFTBRACKET: &str = "{";
pub static RIGHTBRACKET: &str = "}";
pub static EOF: &str = "EOF";
pub static ILLEGAL: &str = "ILLEGAL";
pub static TEXT: &str = "TEXT";
wq2340p['

pub struct Token {
    pub literal: String,
    pub _type: String,
}

impl Token {
    pub fn new(_type : &str, ch : Option<char>) -> Token {
        let mut _literal = match ch {
            None => ' '.to_string(),
            _=> ch.unwrap().to_string()
        };
        Token {
            literal : _literal,
            _type : String::from(_type),
        }
    }
}
