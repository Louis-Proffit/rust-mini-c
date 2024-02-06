use std::fmt::{Display, Formatter};
use crate::rtl::structure::BlockIdent;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtlError<'a> {
    VarNotFound(BlockIdent<'a>),
    DuplicateBlockIdent(BlockIdent<'a>),
}

impl<'a> Display for RtlError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}