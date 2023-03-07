use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::rtl::structure::{BlockIdent, Fresh, Instr};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::PseudoRegister;

#[derive(Debug)]
pub struct Graph<'a> {
    pub vars: HashMap<BlockIdent<'a>, PseudoRegister>,
    pub instrs: HashMap<Label, Instr<'a>>,
}

#[derive(new)]
pub struct DisplayableGraph<'a> {
    graph: &'a Graph<'a>,
    entry: &'a Label,
    exit: &'a Label,
}

#[derive(new)]
pub struct DisplayableVar {
    pub name: String,
    pub register: PseudoRegister,
}

impl<'a> Graph<'a> {
    pub fn new(vars: HashMap<BlockIdent, PseudoRegister>) -> Graph {
        Graph {
            instrs: HashMap::new(),
            vars,
        }
    }

    pub fn locals(&self) -> Vec<DisplayableVar> {
        let mut locals = vec![];
        for (ident, reg) in self.vars.iter() {
            match ident {
                BlockIdent::Arg(_, _) => {}
                BlockIdent::Local(block_index, ident) => {
                    locals.push(DisplayableVar::new(format!("{}_{}", ident, block_index), reg.clone()))
                }
            }
        }

        locals
    }

    pub fn insert_with_label(&mut self, label: Label, instr: Instr<'a>) {
        self.instrs.insert(label.clone(), instr);
    }

    pub fn insert(&mut self, instr: Instr<'a>) -> Label {
        let label = Label::fresh();
        self.instrs.insert(label.clone(), instr);
        label
    }
}

impl DisplayableGraph<'_> {
    fn visit(&self, visited: &mut HashSet<Label>, label: &Label, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !visited.contains(label) && label != self.exit {
            visited.insert(label.clone());
            let instr = self.graph.instrs.get(label).unwrap().clone();
            writeln!(f, "\t{}: {}", label, instr)?;
            match instr {
                Instr::EConst(_, _, l)
                | Instr::ELoad(_, _, _, l)
                | Instr::EStore(_, _, _, l)
                | Instr::EMUnop(_, _, l)
                | Instr::EMBinop(_, _, _, l)
                | Instr::ECall(_, _, _, l)
                | Instr::EGoto(l) => {
                    self.visit(visited, &l, f)?;
                }
                Instr::EMuBranch(_, _, l1, l2)
                | Instr::EMbBranch(_, _, _, l1, l2) => {
                    self.visit(visited, &l1, f)?;
                    self.visit(visited, &l2, f)?;
                }
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


impl Display for DisplayableVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.register)
    }
}