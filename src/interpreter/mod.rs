use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

pub mod typer;
pub mod rtl;

pub const MAIN: &str = "main";
pub const PUTCHAR: &str = "putchar";
pub const MALLOC: &str = "malloc";

#[derive(Clone)]
pub struct Stdout {
    buffer: RefCell<Vec<u8>>,
}

impl Debug for Stdout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Stdout {
    fn to_string(&self) -> String {
        String::from_iter(self.buffer.borrow().iter().map(|index| { *index as char }))
    }
}

impl Stdout {
    fn new() -> Stdout {
        Stdout { buffer: RefCell::new(Vec::new()) }
    }

    pub fn putchar(&self, char: u8) {
        self.buffer.borrow_mut().push(char)
    }
}
