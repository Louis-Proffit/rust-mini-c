pub mod graph;
pub mod label;
pub mod register;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::common::{Ident, Value};
use crate::rtl::structure::graph::{Graph, DisplayableGraph, DisplayableVar};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

#[derive(new, Debug)]
pub struct File<'a> {
    pub funs: HashMap<Ident<'a>, Fun<'a>>,
}

#[derive(new, Debug)]
pub struct Fun<'a> {
    pub name: Ident<'a>,
    pub result: Register,
    pub arguments: Vec<Register>,
    pub locals: HashMap<BlockIdent<'a>, Register>,
    pub entry: Label,
    pub exit: Label,
    pub graph: Graph<'a>,
}

#[derive(Debug, Clone)]
pub enum Instr<'a> {
    EConst(Value, Register, Label),
    ELoad(Register, usize, Register, Label),
    EStore(Register, Register, usize, Label),
    EMUnop(Munop, Register, Label),
    EMBinop(Mbinop, Register, Register, Label),
    EMuBranch(MuBranch, Register, Label, Label),
    EMbBranch(MbBranch, Register, Register, Label, Label),
    ECall(Register, Ident<'a>, Vec<Register>, Label),
    EGoto(Label),
}

#[derive(Debug, Clone)]
pub enum Munop {
    Maddi(Value),
    Msetei(Value),
    Msetnei(Value),
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

struct Registers<'a>(&'a Vec<Register>);

struct DisplayableVars(Vec<DisplayableVar>);

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
        writeln!(f, "{} {}({})", self.result, self.name, Registers(&self.arguments))?;
        writeln!(f, "\tentry : {}", self.entry)?;
        writeln!(f, "\texit : {}", self.exit)?;
        writeln!(f, "\tlocals: {}", DisplayableVars(self.graph.locals()))?;

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
            Instr::ECall(reg, name, args, l) => write!(f, "call {} {}({}) --> {}", reg, name, Registers(args), l),
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

impl Display for Registers<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for reg in self.0 {
            write!(f, "{}", reg)?;
        }
        Ok(())
    }
}


impl Display for DisplayableVars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for var in &self.0 {
            write!(f, "{},", var)?;
        }
        Ok(())
    }
}
