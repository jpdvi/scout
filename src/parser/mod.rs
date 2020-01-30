use super::token;
use super::lexer;
use super::lexer::Lexing;
use super::data;

pub struct Parser {
    l : lexer::Lexer,
    current_token: Option<token::Token>,
    peek_token: Option<token::Token>,
}

impl Parser {
    fn new(input: &str, _data: &data::Data, _pattern: &token::Pattern) -> Self {
        let mut _lexer : lexer::Lexer = lexer::Lexer::new(input, &_data.clone(), &_pattern.clone());  
        Self {
            l: _lexer,
            current_token: None,
            peek_token: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::token;
    use super::data;

    #[test]
    fn test_validator_init(){
        let mut p = token::Pattern::new(token::DOUBLELEFTBRACKET, token::DOUBLERIGHTBRACKET, token::PatternType::HTML);
        let mut d = data::Data::new();
        d.add("test", "data");
        let mut v : Parser = Parser::new("Hello World", &d, &p);
        // Check ownership
        d.add("test2", "scope");
        p._type = token::PatternType::STRING;
    }
}
