use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_getters::Getters;
use derive_new::new;
use crate::rtl::structure::{BlockIdent, Fresh, Instr};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

#[derive(Debug, Getters)]
pub struct Graph {
    vars: RefCell<HashMap<BlockIdent, Register>>,
    instrs: RefCell<HashMap<Label, Instr>>,
}

#[derive(new)]
pub struct PrintableGraph<'a> {
    graph: &'a Graph,
    entry: &'a Label,
    exit: &'a Label,
}

impl Graph {
    pub fn new(vars: HashMap<BlockIdent, Register>) -> Graph {
        Graph {
            instrs: RefCell::new(HashMap::new()),
            vars: RefCell::new(vars),
        }
    }

    pub fn insert_with_label(&self, label: Label, instr: Instr) {
        self.instrs.borrow_mut().insert(label.clone(), instr);
    }

    pub fn insert(&self, instr: Instr) -> Label {
        let label = Label::fresh();
        self.instrs.borrow_mut().insert(label.clone(), instr);
        label
    }
}

impl PrintableGraph<'_> {
    fn visit(&self, visited: &mut HashSet<Label>, label: &Label, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !visited.contains(label) && label != self.exit {
            let instr = self.graph.instrs.borrow().get(label).unwrap().clone();
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

impl Display for PrintableGraph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut visited = HashSet::new();

        self.visit(&mut visited, &self.entry, f)
    }
}
