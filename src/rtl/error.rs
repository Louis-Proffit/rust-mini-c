use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtlError {
    Any(String)
}

impl Display for RtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RtlError::Any(x) => writeln!(f, "{}", x)
        }
    }
}