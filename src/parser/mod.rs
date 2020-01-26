pub struct Template<'a> {
    pub read_position: u32,
    pub position: u32,
    pub text: &'a str,
}

pub trait Reader {
    fn read(&mut self);
    fn read_char(&mut self);
}

impl Reader for Template<'_> {
    fn read(&mut self) {
        while self.read_position as usize <= self.text.len() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.position += 1;
        self.read_position += 1;
    }
}
