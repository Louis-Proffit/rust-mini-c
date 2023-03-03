use std::fmt::{Display, Formatter};
use crate::rtl::structure::BlockIdent;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtlError {
    VarNotFound(BlockIdent),
    DuplicateBlockIdent(BlockIdent),
}

impl Display for RtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}