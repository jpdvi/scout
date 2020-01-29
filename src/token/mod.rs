pub type TokenType<'a> = &'a str;

// Templating
pub static DOUBLELEFTBRACKET: &str ="{{";
pub static DOUBLERIGHTBRACKET: &str = "}}";
// Generic
pub static EOF: &str = "EOF";
pub static ILLEGAL: &str = "ILLEGAL";
pub static TEXT: &str = "TEXT";
pub static SPACE: &str = "SPACE";
// Tokens
pub static LEFTBRACKET: &str = "{";
pub static RIGHTBRACKET: &str = "}";
// Operators
pub static LT: &str = "<";
pub static GT: &str = ">";
// HTML
pub static ENDHTMLBLOCK: &str = "/>";

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

pub enum PatternType {
    HTML,
    STRING,
}

pub struct Pattern {
    pub _type: PatternType,
    pub left: String,
    pub right: String
}

impl Pattern {
    pub fn new(left: TokenType, right: TokenType, _type: PatternType) -> Self {
        Self {
            left: String::from(left),
            right: String::from(right),
            _type: _type,
        }
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let t : PatternType = match &self._type {
            PatternType::HTML => PatternType::HTML,
            _=> PatternType::STRING
        };
        Self {
            left: String::from(&self.left),
            right: String::from(&self.right),
            _type: t,
        }
    }
}
