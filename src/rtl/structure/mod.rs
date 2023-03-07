pub mod graph;
pub mod label;
pub mod register;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::common::{Ident, Value};
use crate::rtl::structure::graph::{Graph, DisplayableGraph};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::PseudoRegister;
use crate::utils::DisplayableVec;

#[derive(new, Debug)]
pub struct File<'a> {
    pub funs: HashMap<Ident<'a>, Fun<'a>>,
}

#[derive(new, Debug)]
pub struct Fun<'a> {
    pub name: Ident<'a>,
    pub result: PseudoRegister,
    pub arguments: Vec<PseudoRegister>,
    pub locals: HashMap<BlockIdent<'a>, PseudoRegister>,
    pub entry: Label,
    pub exit: Label,
    pub graph: Graph<'a>,
}

#[derive(Debug, Clone)]
pub enum Instr<'a> {
    EConst(Value, PseudoRegister, Label),
    ELoad(PseudoRegister, usize, PseudoRegister, Label),
    EStore(PseudoRegister, PseudoRegister, usize, Label),
    EMUnop(Munop, PseudoRegister, Label),
    EMBinop(Mbinop, PseudoRegister, PseudoRegister, Label),
    EMuBranch(MuBranch, PseudoRegister, Label, Label),
    EMbBranch(MbBranch, PseudoRegister, PseudoRegister, Label, Label),
    ECall(PseudoRegister, Ident<'a>, Vec<PseudoRegister>, Label),
    EGoto(Label),
}

#[derive(Debug, Clone)]
pub enum Munop {
    Maddi(Value),
    Msetei(Value),
    Msetnei(Value),
}

#[derive(Debug, Clone, PartialEq)]
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
    MJlei(Value),
    MJgi(Value),
}

#[derive(Debug, Clone)]
pub enum MbBranch {
    MJl,
    MJle,
}

pub type BlockIdent<'a> = crate::typer::structure::BlockIdent<'a>;

pub trait Fresh {
    type Item;

    fn fresh() -> Self::Item;
}

impl Display for File<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== RTL ==================================================")?;
        for (_, fun) in &self.funs {
            writeln!(f, "{}", fun)?;
        }
        Ok(())
    }
}

impl Display for Fun<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}({})", self.result, self.name, DisplayableVec(&self.arguments))?;
        writeln!(f, "\tentry : {}", self.entry)?;
        writeln!(f, "\texit : {}", self.exit)?;
        writeln!(f, "\tlocals: {}", DisplayableVec(&self.graph.locals()))?;

        let printable_graph = DisplayableGraph::new(
            &self.graph,
            &self.entry,
            &self.exit,
        );

        write!(f, "{}", printable_graph)?;
        Ok(())
    }
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::EConst(c, r, l) => write!(f, "mov ${} {} --> {}", c, r, l),
            Instr::ELoad(address, offset, dest, l) => write!(f, "load {}({}) to {} --> {}", address, offset, dest, l),
            Instr::EStore(address, value, offset, l) => write!(f, "store {} in {}({}) --> {}", value, address, offset, l),
            Instr::EMUnop(op, r, l) => write!(f, "{} {} --> {}", op, r, l),
            Instr::EMBinop(op, r1, r2, l) => write!(f, "{} {} {} --> {}", op, r1, r2, l),
            Instr::EMuBranch(op, reg, lbl1, lbl2) => write!(f, "{} {} --> {},{}", op, reg, lbl1, lbl2),
            Instr::EMbBranch(op, r1, r2, l1, l2) => write!(f, "bbranch {} : {} {} --> {},{}", op, r1, r2, l1, l2),
            Instr::ECall(reg, name, args, l) => write!(f, "call {} {}({}) --> {}", reg, name, DisplayableVec(args), l),
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

impl Display for MbBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MbBranch::MJl => write!(f, "jl"),
            MbBranch::MJle => write!(f, "jle")
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