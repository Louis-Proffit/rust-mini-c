use crate::rtl::structure::label::Label;

#[derive(Debug)]
pub enum RtlInterpreterError {
    FunctionDoesNotExist(String),
    NoInstructionForDefaultFunction(Label),
    NoSuchInstruction(Label),
    Other(&'static str),
}
