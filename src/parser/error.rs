use nom::error::Error;
use crate::parser::Input;

#[derive(Debug)]
pub enum ParserError<'a> {
    Nom(Error<Input<'a>>),
    Any(&'static str),
}