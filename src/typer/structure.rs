use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use derive_new::new;
use derive_getters::Getters;
use crate::common::{Value, Ident, MALLOC, PUTCHAR, MAIN, StackOffset};

pub type StructSize = Value;
pub type Unop = crate::parser::structure::Unop;
pub type Binop = crate::parser::structure::Binop;

#[derive(new, Debug, Getters)]
pub struct File<'a> {
    funs: HashMap<Ident<'a>, Fun<'a>>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum BlockIdent<'a> {
    Arg(usize, Ident<'a>),
    Local(u8, Ident<'a>),
}

#[derive(new, Debug, Getters)]
pub struct Signature<'a> {
    name: Ident<'a>,
    typ: Typ<'a>,
    args: Vec<Formal<'a>>,
}

#[derive(new, Debug, Getters)]
pub struct Fun<'a> {
    signature: Rc<Signature<'a>>,
    locals: HashSet<BlockIdent<'a>>,
    block: Block<'a>,
}

#[derive(new, Debug, Getters, Clone)]
pub struct Formal<'a> {
    name: BlockIdent<'a>,
    typ: Typ<'a>,
}

#[derive(new, Debug, Getters)]
pub struct Field<'a> {
    name: Ident<'a>,
    index: u8,
    typ: Typ<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Typ<'a> {
    TInt,
    TVoidStar,
    TTypeNull,
    TStruct(Rc<Struct<'a>>),
}

#[derive(new, Debug, Getters)]
pub struct Struct<'a> {
    name: Ident<'a>,
    fields: Rc<RefCell<HashMap<Ident<'a>, Rc<Field<'a>>>>>, // TODO remove refcell
}

#[derive(new, Debug, Getters)]
pub struct Block<'a> {
    stmts: Vec<Stmt<'a>>,
}

#[derive(Debug)]
pub enum Stmt<'a> {
    SSkip,
    SExpr(Expr<'a>),
    SIf(Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
    SWhile(Expr<'a>, Box<Stmt<'a>>),
    SBlock(Block<'a>),
    SReturn(Expr<'a>),
}

#[derive(new, Debug, Getters)]
pub struct Expr<'a> {
    node: ExprNode<'a>,
    typ: Typ<'a>,
}

#[derive(new, Getters, Debug)]
pub struct ArgExpr<'a> {
    formal: Formal<'a>,
    expr: Expr<'a>,
}

#[derive(Debug)]
pub enum ExprNode<'a> {
    EConst(Value),
    EAccessLocal(BlockIdent<'a>),
    EAccessField(Box<Expr<'a>>, Rc<Field<'a>>),
    EAssignLocal(BlockIdent<'a>, Box<Expr<'a>>),
    EAssignField(Box<Expr<'a>>, Rc<Field<'a>>, Box<Expr<'a>>),
    EUnop(Unop, Box<Expr<'a>>),
    EBinop(Binop, Box<Expr<'a>>, Box<Expr<'a>>),
    ECall(Rc<Signature<'a>>, Vec<ArgExpr<'a>>),
}

impl Struct<'_> {
    const FIELD_SIZE: usize = 8;

    pub fn c_size(&self) -> StructSize {
        (self.fields.borrow().len() * Struct::FIELD_SIZE) as Value
    }
}

impl Field<'_> {
    pub fn c_offset(&self) -> StackOffset {
        (self.index * 8) as StackOffset
    }
}

impl<'a> File<'a> {
    pub fn into_funs(self) -> HashMap<Ident<'a>, Fun<'a>> {
        self.funs
    }
}

impl PartialEq<Self> for Struct<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Signature<'_> {
    pub fn main<'a>() -> Signature<'a> {
        Signature::new(MAIN, Typ::TInt, vec![])
    }

    pub fn putchar<'a>() -> Signature<'a> {
        Signature::new(PUTCHAR, Typ::TInt, vec![Formal::new(BlockIdent::Arg(0, "c"), Typ::TInt)])
    }

    pub fn malloc<'a>() -> Signature<'a> {
        Signature::new(MALLOC, Typ::TVoidStar, vec![Formal::new(BlockIdent::Arg(0, "n"), Typ::TInt)])
    }
}
