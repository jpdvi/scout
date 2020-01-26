mod parser;
mod token;
use parser::Template;
use parser::Lexer;
use std::collections::HashMap;

fn main() {
    let st: &str = "my new template {butt}";
    let data : HashMap<&str, &str> = [
        ("butt", "ass"),
    ].iter().cloned().collect();

    let mut temp = Template::new(st, data);
    temp.read();
}
