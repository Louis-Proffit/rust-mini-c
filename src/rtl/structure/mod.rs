pub mod graph;
pub mod label;
pub mod register;

use std::fmt::{Display, Formatter};
use derive_new::new;
use derive_getters::Getters;
use crate::rtl::structure::graph::{Graph, PrintableGraph};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

pub type Const = crate::typer::structure::Const;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum BlockIdent {
    Arg(usize, String),
    Local(u8, String),
}

pub trait Fresh {
    type Item;

    fn fresh() -> Self::Item;
}

#[derive(new, Getters)]
pub struct Fun {
    name: String,
    result: Register,
    entry: Label,
    exit: Label,
    graph: Graph,
}

#[derive(new, Getters)]
pub struct File {
    funs: Vec<Fun>,
}

#[derive(Debug, Clone)]
pub enum Instr {
    EConst(Const, Register, Label),
    ELoad(Register, u8, Register, Label),
    EStore(Register, Register, u8, Label),
    EMUnop(Munop, Register, Label),
    EMBinop(Mbinop, Register, Register, Label),
    EMuBranch(MuBranch, Register, Label, Label),
    EMbBranch(MbBranch, Register, Register, Label, Label),
    ECall(Register, String, Vec<Register>, Label),
    EGoto(Label),
}

#[derive(Debug, Clone)]
pub enum Munop {
    Maddi(Const),
    Msetei(Const),
    Msetnei(Const),
}

#[derive(Debug, Clone)]
pub enum Mbinop {
    MMov,
    MAdd,
    MSub,
    MMul,
    MDiv,
    MSete,
    MSetne,
    Msetl,
    Msetle,
    Msetg,
    Msetge,
}

#[derive(Debug, Clone)]
pub enum MuBranch {
    MJz,
    MJnz,
    MJlei(Const),
    MJgi(Const),
}

#[derive(Debug, Clone)]
pub enum MbBranch {
    MJl,
    MJle,
}

impl<'a> From<crate::typer::structure::BlockIdent<'a>> for BlockIdent {
    fn from(value: crate::typer::structure::BlockIdent<'a>) -> Self {
        match value {
            crate::typer::structure::BlockIdent::Arg(x, y) => BlockIdent::Arg(x, String::from(y)),
            crate::typer::structure::BlockIdent::Local(x, y) => BlockIdent::Local(x, String::from(y))
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== RTL ==================================================")?;
        for fun in &self.funs {
            writeln!(f, "{fun}\n")?;
        }
        Ok(())
    }
}

impl Display for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}({:?})", self.result, self.name, self.graph.arguments())?;
        writeln!(f, "\tentry : {}", self.entry)?;
        writeln!(f, "\texit : {}", self.exit)?;
        writeln!(f, "\tlocals: {:?}", self.graph.locals())?;

        let printable_graph = PrintableGraph::new(
            &self.graph,
            &self.entry,
            &self.exit,
        );
        writeln!(f, "{}", printable_graph)?;
        Ok(())
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::EConst(c, r, l) => write!(f, "mov ${} {} --> {}", c, r, l),
            Instr::ELoad(_, _, _, _) => write!(f, "load TODO"),
            Instr::EStore(_, _, _, _) => write!(f, "store TODO"),
            Instr::EMUnop(op, r, l) => write!(f, "{} {} --> {}", op, r, l),
            Instr::EMBinop(op, r1, r2, l) => write!(f, "{} {} {} --> {}", op, r1, r2, l),
            Instr::EMuBranch(op, reg, lbl1, lbl2) => write!(f, "{} {} --> {},{}", op, reg, lbl1, lbl2),
            Instr::EMbBranch(_, _, _, _, _) => write!(f, "bbranch TODO"),
            Instr::ECall(reg, name, args, l) => write!(f, "call {} {}({:?}) --> {}", reg, name, args, l),
            Instr::EGoto(l) => write!(f, "goto {}", l),
        }
    }
}

impl Display for Munop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Munop::Maddi(c) => write!(f, "add ({c})"),
            Munop::Msetei(c) => write!(f, "sete ({c})"),
            Munop::Msetnei(c) => write!(f, "setne ({c})"),
        }
    }
}

impl Display for Mbinop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mbinop::MMov => write!(f, "movq"),
            Mbinop::MAdd => write!(f, "addq"),
            Mbinop::MSub => write!(f, "subq"),
            Mbinop::MMul => write!(f, "mulq"),
            Mbinop::MDiv => write!(f, "divq"),
            Mbinop::MSete => write!(f, "sete"),
            Mbinop::MSetne => write!(f, "setne"),
            Mbinop::Msetl => write!(f, "setl"),
            Mbinop::Msetle => write!(f, "setle"),
            Mbinop::Msetg => write!(f, "setg"),
            Mbinop::Msetge => write!(f, "setge"),
        }
    }
}

impl Display for MuBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MuBranch::MJz => write!(f, "jz"),
            MuBranch::MJnz => write!(f, "jnz"),
            MuBranch::MJlei(c) => write!(f, "jle {}", c),
            MuBranch::MJgi(c) => write!(f, "jg {}", c),
        }
    }
}
