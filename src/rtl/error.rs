use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RtlError {
    Any(&'static str)
}

impl Display for RtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RtlError::Any(x) => writeln!(f, "{}", x)
        }
    }
}