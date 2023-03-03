use derive_new::new;
use derive_getters::Getters;
use crate::common::{Value, Ident};

#[derive(new, Debug, PartialEq, Getters)]
pub struct File<'a> {
    funs: Vec<Fun<'a>>,
    structs: Vec<Struct<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Typ<'a> {
    TInt,
    TStruct(Ident<'a>),
}

#[derive(new, Debug, PartialEq, Getters)]
pub struct Formal<'a> {
    name: Ident<'a>,
    typ: Typ<'a>,
}

#[derive(new, Debug, PartialEq, Getters)]
pub struct Struct<'a> {
    name: Ident<'a>,
    fields: Vec<Formal<'a>>,
}

#[derive(new, Debug, PartialEq, Getters)]
pub struct Fun<'a> {
    profile: Formal<'a>,
    args: Vec<Formal<'a>>,
    body: Block<'a>,
}

#[derive(new, Debug, PartialEq, Getters)]
pub struct Block<'a> {
    vars: Vec<Formal<'a>>,
    stmts: Vec<Stmt<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    SSkip,
    SExpr(Expr<'a>),
    SIf(Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
    SWhile(Expr<'a>, Box<Stmt<'a>>),
    SBlock(Block<'a>),
    SReturn(Expr<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    EConst(Value),
    EVar(Ident<'a>),
    EArrow(Box<Expr<'a>>, Ident<'a>),
    EAssign(Box<Expr<'a>>, Box<Expr<'a>>),
    EUnop(Unop, Box<Expr<'a>>),
    EBinop(Binop, Box<Expr<'a>>, Box<Expr<'a>>),
    ECall(Box<Expr<'a>>, Vec<Expr<'a>>),
    ESizeof(Ident<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unop {
    UNot,
    UMinus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binop {
    BEq,
    BNeq,
    BLt,
    BGt,
    BGe,
    BLe,
    BAdd,
    BSub,
    BMul,
    BDiv,
    BAnd,
    BOr,
}
