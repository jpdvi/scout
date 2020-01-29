pub type TokenType<'a> = &'a str;

pub static LEFTBRACKET: &str = "{";
pub static DOUBLELEFTBRACKET: &str ="{{";
pub static RIGHTBRACKET: &str = "}";
pub static DOUBLERIGHTBRACKET: &str = "}}";
pub static EOF: &str = "EOF";
pub static ILLEGAL: &str = "ILLEGAL";
pub static TEXT: &str = "TEXT";
pub static SPACE: &str = "SPACE";

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

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            _type : String::from(&self._type),
            literal : String::from(&self.literal)
        }
        }
}

pub struct Pattern {
    pub left: String,
    pub right: String
}

impl Pattern {
    pub fn new(left: TokenType, right: TokenType) -> Self {
        Self {
            left: String::from(left),
            right: String::from(right)
        }
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Self {
            left: String::from(&self.left),
            right: String::from(&self.right)
        }
    }
}
