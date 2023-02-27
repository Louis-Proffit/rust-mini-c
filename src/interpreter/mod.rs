use std::fmt::{Debug, Formatter};

pub mod typer;

#[derive(Clone)]
pub struct Stdout {
    buffer: Vec<u8>,
}

impl Debug for Stdout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Stdout {
    fn to_string(&self) -> String {
        String::from_iter(self.buffer.iter().map(|index| { *index as char }))
    }
}

impl Stdout {
    fn new() -> Stdout {
        Stdout { buffer: Vec::new() }
    }

    pub fn putchar(&mut self, char: u8) {
        self.buffer.push(char)
    }
}
