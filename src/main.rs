mod parser;
mod token;
use parser::Reader;

fn main() {
    let st: &str = "my new template, james {bond}";
    let mut reader = parser::Template {
        text: st,
        read_position: 0,
        position: 0,
    };

    reader.read();
}
