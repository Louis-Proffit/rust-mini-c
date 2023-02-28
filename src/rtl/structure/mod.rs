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

#[derive(Eq, Hash, PartialEq, new, Getters, Debug, Clone)]
pub struct BlockIdent {
    name: String,
    block_index: u8,
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
        BlockIdent::new(
            String::from(*value.name()),
            *value.block_index(),
        )
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
        writeln!(f, "{} {}", self.result, self.name)?;
        writeln!(f, "\tentry : {}", self.entry)?;
        writeln!(f, "\texit : {}", self.exit)?;
        writeln!(f, "\tlocals: TODO")?;

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
            Instr::ELoad(_, _, _, _) => todo!(),
            Instr::EStore(_, _, _, _) => todo!(),
            Instr::EMUnop(_, _, _) => todo!(),
            Instr::EMBinop(_, _, _, _) => todo!(),
            Instr::EMuBranch(op, reg, lbl1, lbl2) => write!(f, "{} {} --> {},{}", op, reg, lbl1, lbl2),
            Instr::EMbBranch(_, _, _, _, _) => todo!(),
            Instr::ECall(_, _, _, _) => todo!(),
            Instr::EGoto(_) => todo!(),
        }
    }
}

impl Display for MuBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MuBranch::MJz => write!(f, "jz"),
            MuBranch::MJnz => todo!(),
            MuBranch::MJlei(_) => todo!(),
            MuBranch::MJgi(_) => todo!(),
        }
    }
}
