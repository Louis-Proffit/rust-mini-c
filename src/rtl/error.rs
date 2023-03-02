use crate::rtl::structure::BlockIdent;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtlError {
    VarNotFound(BlockIdent),
    DuplicateBlockIdent(BlockIdent),
}