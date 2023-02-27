pub mod typer;

#[derive(Clone)]
pub struct Stdout {
    buffer: Vec<u8>,
}

impl Stdout {
    fn new() -> Stdout {
        Stdout { buffer: Vec::new() }
    }

    pub fn putchar(&mut self, char: u8) {
        self.buffer.push(char)
    }

    pub fn to_str(self) -> String {
        String::from_iter(self.buffer.into_iter().map(|index| { index as char }))
    }
}
