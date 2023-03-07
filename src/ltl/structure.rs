use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::common::{Ident, Value};
use crate::ertl::structure::{Label, MbBranch, Mbinop, MuBranch, Munop};
use crate::ertl::structure::register::{PhysicalRegister, Register};

#[derive(Debug, new)]
pub struct File<'a> {
    funs: HashMap<Ident<'a>, Fun<'a>>,
}

#[derive(Debug, new)]
pub struct Fun<'a> {
    name: Ident<'a>,
    entry: Label,
    body: Graph<'a>,
}

#[derive(Debug, new)]
pub struct Graph<'a> {
    instrs: HashMap<Label, Instr<'a>>,
}

#[derive(Debug)]
pub enum Instr<'a> {
    ELoad(Register, u8, Register, Label),
    EStore(Register, Register, u8, Label),
    EGoto(Label),
    EReturn,
    EConst(Value, Operand, Label),
    EMunop(Munop, Operand, Label),
    EMBinop(Mbinop, Operand, Operand, Label),
    EMuBranch(MuBranch, Operand, Label, Label),
    EMbBranch(MbBranch, Operand, Operand, Label, Label),
    EPush(Operand, Label),
    ECall(Ident<'a>, Label),
    EPop(Operand, Label),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operand {
    Reg(PhysicalRegister),
    Spilled(u16),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(x) => write!(f, "{}", x),
            Operand::Spilled(x) => write!(f, "%rsp({})", x)
        }
    }
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::ELoad(addr, offset, dest, l) => write!(f, "load {}({}) in {} --> {}", addr, offset, dest, l),
            Instr::EStore(addr, value, offset, l) => write!(f, "store {} in {}({}) --> {}", value, addr, offset, l),
            Instr::EGoto(l) => write!(f, "goto {}", l),
            Instr::EReturn => write!(f, "return"),
            Instr::EConst(c, op, l) => write!(f, "mov ${} {} --> {}", c, op, l),
            Instr::EMunop(op, r, l) => write!(f, "{} {} --> {}", op, r, l),
            Instr::EMBinop(op, r1, r2, l) => write!(f, "{} {},{} --> {}", op, r1, r2, l),
            Instr::EMuBranch(op, ope, l1, l2) => write!(f, "{} {} --> {},{}", op, ope, l1, l2),
            Instr::EMbBranch(op, ope1, ope2, l1, l2) => write!(f, "{} {},{} --> {},{}", op, ope1, ope2, l1, l2),
            Instr::EPush(op, l) => write!(f, "push {} --> {}", op, l),
            Instr::ECall(name, l) => write!(f, "call {} --> {}", name, l),
            Instr::EPop(reg, l) => write!(f, "pop {} --> {}", reg, l),
        }
    }
}

impl Display for Fun<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "\tentry: {}", self.entry)?;
        write!(f, "{}", self.body)
    }
}

impl Display for Graph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (label, instr) in &self.instrs {
            writeln!(f, "\t{}: {}", label, instr)?;
        }
        Ok(())
    }
}

impl Display for File<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (_name, fun) in &self.funs {
            writeln!(f, "{}", fun)?;
        }
        Ok(())
    }
}