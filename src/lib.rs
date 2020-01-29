mod token;
mod data;
mod lexer;
mod parser;
use lexer::Lexing;


pub struct Mark {
    pub left_index : Option<usize>,
    pub right_index: Option<usize>,
}


impl Mark {
    fn new() -> Self {
        Self {
            left_index : None,
            right_index : None,
        }
    }
}


pub struct Template<'a> {
    l : lexer::Lexer<'a>,
    pub input: &'a str,
    pub pattern: token::Pattern,
    pub tokens : Vec<token::Token>,
    marks : Vec<Mark>,
}

impl<'a> Template<'a> {
    pub fn new(input: &'a str, 
        data: data::Data, 
        left_delimiter : token::TokenType, 
        right_delimiter: token::TokenType) -> Self {
        let _pattern : token::Pattern = token::Pattern::new(left_delimiter, right_delimiter, token::PatternType::STRING);
        let mut template = Self { 
            l : lexer::Lexer::new(input, &data, &_pattern),
            input: input,
            pattern: _pattern.clone(),
            marks : vec![],
            tokens : vec![],
        };
        return template
    }

    pub fn read_mark_content(&self, left_index: usize, right_index: usize) -> String { 
        let x = &self.tokens[left_index  .. right_index];
        let content_tokens  = x.iter().filter(|item| {
            item._type != self.pattern.left && item._type != self.pattern.right
        });
        let mut content : String = String::new();
        for token in content_tokens {
            content.push_str(&String::from(&token.literal));
        }
        return content
    }

    pub fn compile(&mut self) -> Result<String, &str> {
        self.l.read();
        self.tokens = self.l.tokens.clone();
        let mut mark = Mark::new();
        for (i, item) in self.tokens.iter().enumerate() {
            if item._type == self.pattern.left && mark.left_index == None {
                mark.left_index = Some(i);
            }
            else if item._type == self.l.pattern.right && mark.right_index == None {
                mark.right_index = Some(i);
            }
            else if mark.left_index != None && mark.right_index != None {
               self.marks.push(mark); 
               mark = Mark::new();
            }
        }

        let mut removed : usize = 0; // Keeps track of removed tokens
        let mut formatted_content : String = String::new(); // Final content tokens
        for (i, item) in self.marks.iter().enumerate() {
            // Grab indexes from mark and decrement to account for removed items
            let mut x : usize = item.left_index.unwrap() - removed;
            let y : usize = item.right_index.unwrap() - removed;
            // Plain text string of the value within a pattern
            let mut data_key : String = self.read_mark_content(x, y);
            // Content from Data map
            let mut content : String = String::new();
            // Check if data_key exists in our self.data (Data) map
            if self.l.data.data.contains_key(&data_key.to_string()) {
                content.push_str(&self.l.data.data[&data_key.to_string()]);
            } else {
                return Err("Unknown data key")
            }
            // Keep track of removed item count in this iteration
            let mut temp : usize = 0;
            while x <= y {
                // Remove token at x minus any tokens we just removed
                self.tokens.remove(x - temp);
                temp += 1;
                x += 1;
                
            }

            // Split and merge self.tokens to include our new content
            let mut section_end: usize = item.left_index.unwrap() - removed;
            let mut left_arr = &self.tokens[ .. section_end].to_vec();                        
            let mut right_arr = &self.tokens[ section_end .. ].to_vec();
            let mut content_token = token::Token::new(token::TEXT, None);
            content_token.literal = content;
            let mut updated_tokens = vec![]; 
            for item in left_arr {
                updated_tokens.push(item.clone());
            }
            updated_tokens.push(content_token);
            for item in right_arr {
                updated_tokens.push(item.clone());
            }
            self.tokens = updated_tokens;
            removed += temp -1;
        }
        
        for item in &self.tokens {
            if item._type != token::EOF {
                formatted_content.push_str(&String::from(&item.literal));
            }
        }
        return Ok(formatted_content);
    }
}



#[cfg(test)]
mod tests {
    use super::token;
    use super::Template;
    use super::data::Data;
    use super::lexer;
    use super::lexer::Lexing;

    
    #[test]
    fn test_template_create() {
        let st: &str = "my new template {test}";
        let mut d = Data::new();
        let mut t = Template::new(st, d, token::LEFTBRACKET, token::RIGHTBRACKET);
    }

    #[test]
    fn test_compile() {
        let st: &str = "my {{one}} new template {{two}}";
        let test_string : &str = "my two new template three";
        let mut d = Data::new();
        d.add_many(vec![("one","two"), ("two", "three")]);
        let mut t = Template::new(st, d, token::DOUBLELEFTBRACKET, token::DOUBLERIGHTBRACKET);
        let result = t.compile();
        assert_eq!(result.unwrap(), test_string);
        assert_eq!(t.input, st);
        assert_eq!(t.l.data.data.len() as u32, 2); 
    }

    #[test]
    fn test_read() {
        let st: &str = "my {{one}} {new template {{two}}";
        let mut d = Data::new();
        d.add_many(vec![("one","two"), ("two", "three")]);
        let mut t = Template::new(st, d, token::DOUBLELEFTBRACKET, token::DOUBLERIGHTBRACKET);
        let v = vec![
        ("m", token::TEXT),
        ("y", token::TEXT),
        (" ", token::SPACE),
        ("{{", token::DOUBLELEFTBRACKET),
        ("o", token::TEXT),
        ("n", token::TEXT),
        ("e", token::TEXT),
        ("}}", token::DOUBLERIGHTBRACKET),
        (" ", token::SPACE),
        ("{", token::TEXT),
        ("n", token::TEXT),
        ("e", token::TEXT),
        ("w", token::TEXT),
        (" ", token::SPACE),
        ("t", token::TEXT),
        ("e", token::TEXT),
        ("m", token::TEXT),
        ("p", token::TEXT),
        ("l", token::TEXT),
        ("a", token::TEXT),
        ("t", token::TEXT),
        ("e", token::TEXT),
        (" ", token::SPACE),
        ("{{", token::DOUBLELEFTBRACKET),
        ("t", token::TEXT),
        ("w", token::TEXT),
        ("o", token::TEXT),
        ("}}", token::DOUBLERIGHTBRACKET),
        (" ", token::EOF),
        ];
        for test_case in v {
            let n  : token::Token = t.l.next_token();
            assert_eq!(test_case.0, n.literal);
            assert_eq!(test_case.1, n._type);
        }
    }
}
