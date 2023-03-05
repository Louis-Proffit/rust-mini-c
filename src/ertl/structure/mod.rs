pub mod register;

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::common::{Ident, Value};
use crate::ertl::structure::register::Register;
use crate::rtl::structure::Fresh;
use crate::utils::DisplayableSet;

pub type Label = crate::rtl::structure::label::Label;
pub type Munop = crate::rtl::structure::Munop;
pub type Mbinop = crate::rtl::structure::Mbinop;
pub type MuBranch = crate::rtl::structure::MuBranch;
pub type MbBranch = crate::rtl::structure::MbBranch;

#[derive(Debug)]
pub enum Instr<'a> {
    EConst(Value, Register, Label),
    ELoad(Register, usize, Register, Label),
    EStore(Register, Register, usize, Label),
    EMUnop(Munop, Register, Label),
    EMBinop(Mbinop, Register, Register, Label),
    EMuBranch(MuBranch, Register, Label, Label),
    EMbBranch(MbBranch, Register, Register, Label, Label),
    ECall(Ident<'a>, u8, Label),
    EGoto(Label),
    EAllocFrame(Label),
    EDeleteFrame(Label),
    EGetParam(u8, Register, Label),
    EPushParam(Register, Label),
    EReturn,
}

#[derive(new, Debug)]
pub struct Graph<'a> {
    pub instrs: HashMap<Label, Instr<'a>>,
}

#[derive(new, Debug)]
pub struct Fun<'a> {
    pub name: Ident<'a>,
    pub argument_count: u8,
    pub locals: HashSet<Register>,
    pub entry: Label,
    pub body: Graph<'a>,
}

#[derive(new, Debug)]
pub struct File<'a> {
    pub funs: HashMap<Ident<'a>, Fun<'a>>,
}

#[derive(new)]
struct DisplayableGraph<'a> {
    graph: &'a Graph<'a>,
    entry: &'a Label,
}

impl<'a> Graph<'a> {
    pub(crate) fn insert_at_label(&mut self, label: &Label, instr: Instr<'a>) {
        self.instrs.insert(label.clone(), instr);
    }

    pub(crate) fn insert(&mut self, instr: Instr<'a>) -> Label {
        let label = Label::fresh();
        self.instrs.insert(label.clone(), instr);
        label
    }
}

impl Display for File<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== ERTL =================================================")?;
        for (_ident, fun) in &self.funs {
            writeln!(f, "{}", fun)?;
        }
        Ok(())
    }
}

impl Display for Fun<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}({})", self.name, self.argument_count)?;
        writeln!(f, "\tentry: {}", self.entry)?;
        writeln!(f, "\tlocals: {}", DisplayableSet(&self.locals))?;
        writeln!(f, "{}", DisplayableGraph::new(&self.body, &self.entry))
    }
}

impl DisplayableGraph<'_> {
    fn visit(&self, visited: &mut HashSet<Label>, label: &Label, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !visited.contains(label) {
            visited.insert(label.clone());
            let instr = self.graph.instrs.get(label).unwrap().clone();
            writeln!(f, "\t{}: {}", label, instr)?;
            match instr {
                Instr::EConst(_, _, l)
                | Instr::ELoad(_, _, _, l)
                | Instr::EStore(_, _, _, l)
                | Instr::EMUnop(_, _, l)
                | Instr::EMBinop(_, _, _, l)
                | Instr::ECall(_, _, l)
                | Instr::EGoto(l)
                | Instr::EAllocFrame(l)
                | Instr::EDeleteFrame(l)
                | Instr::EGetParam(_, _, l)
                | Instr::EPushParam(_, l) => {
                    self.visit(visited, &l, f)?;
                }
                Instr::EMuBranch(_, _, l1, l2)
                | Instr::EMbBranch(_, _, _, l1, l2) => {
                    self.visit(visited, &l1, f)?;
                    self.visit(visited, &l2, f)?;
                }
                Instr::EReturn => {}
            }
        }
        Ok(())
    }
}

impl Display for DisplayableGraph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut visited = HashSet::new();

        self.visit(&mut visited, &self.entry, f)
    }
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::EConst(c, r, l) => write!(f, "mov ${} {} --> {}", c, r, l),
            Instr::ELoad(a, o, r, l) => write!(f, "load {}({}) in {} --> {}", a, o, r, l),
            Instr::EStore(a, v, o, l) => write!(f, "store {} in {}({}) --> {}", v, a, o, l),
            Instr::EMUnop(op, r, l) => write!(f, "{} {} --> {}", op, r, l),
            Instr::EMBinop(op, r1, r2, l) => write!(f, "{} {} {} --> {}", op, r1, r2, l),
            Instr::EMuBranch(op, r, l1, l2) => write!(f, "{} {} --> {}, {}", op, r, l1, l2),
            Instr::EMbBranch(op, r1, r2, l1, l2) => write!(f, "{} {} {} --> {}, {}", op, r1, r2, l1, l2),
            Instr::ECall(name, args, l) => write!(f, "call {}({}) --> {}", name, args, l),
            Instr::EGoto(l) => write!(f, "goto {}", l),
            Instr::EAllocFrame(l) => write!(f, "alloc_frame --> {}", l),
            Instr::EDeleteFrame(l) => write!(f, "delete_frame --> {}", l),
            Instr::EGetParam(o, r, l) => write!(f, "{} {} --> {}", o, r, l),
            Instr::EPushParam(r, l) => write!(f, "push {} --> {}", r, l),
            Instr::EReturn => write!(f, "return")
        }
    }
}