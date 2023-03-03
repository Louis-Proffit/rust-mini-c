use std::fmt::{Display, Formatter};
use crate::common::Value;
use crate::rtl::structure::label::Label;

#[derive(Debug)]
pub enum RtlInterpreterError {
    FunctionDoesNotExist(String),
    NoInstructionForDefaultFunction(Label),
    NoSuchInstruction(Label),
    UnallocatedMemory(Value),
    Other(&'static str),
}


impl Display for RtlInterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}