pub static LEFTBRACKET: &str = "{";
pub static RIGHTBRACKET: &str = "}";

pub struct Token<'a> {
    pub literal: &'a str,
    pub _type: &'a str,
}

pub struct Pattern<'a> {
    pub left: Token<'a>,
    pub right: Token<'a>,
}
