use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

pub type Value = i64;
pub type Ident<'a> = &'a str;
pub type StackOffset = i16;

pub const MAIN: &str = "main";
pub const PUTCHAR: &str = "putchar";
pub const MALLOC: &str = "malloc";

#[derive(Clone)]
pub struct Stdout {
    buffer: RefCell<Vec<char>>,
}

impl Display for Stdout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_iter(self.buffer.borrow().iter()))
    }
}

impl Debug for Stdout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Stdout {
    pub(crate) fn new() -> Stdout {
        Stdout { buffer: RefCell::new(Vec::new()) }
    }

    pub fn putchar(&self, char: char) {
        self.buffer.borrow_mut().push(char)
    }
}


pub mod bool {
    use crate::common::Value;

    pub trait Bool {
        fn to_bool(&self) -> bool;
    }

    impl Bool for Value {
        fn to_bool(&self) -> bool {
            *self != 0
        }
    }

    pub trait ToCBool {
        fn to_minic_bool(&self) -> Value;
    }

    impl ToCBool for bool {
        fn to_minic_bool(&self) -> Value {
            if *self { 1 } else { 0 }
        }
    }
}