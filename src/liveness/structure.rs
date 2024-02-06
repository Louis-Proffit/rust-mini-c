use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::ertl::structure::{Instr, Label};
use crate::ertl::structure::register::Register;
use crate::utils::{DisplayableSet, DisplayableVec};

pub struct LivenessGraph<'a> {
    pub infos: HashMap<Label, LivenessInfo<'a>>,
}

#[derive(Debug, new)]
pub struct LivenessInfo<'a> {
    pub instr: &'a Instr<'a>,
    pub succ: Vec<Label>,
    pub pred: HashSet<Label>,
    pub defs: Vec<Register>,
    pub uses: Vec<Register>,
    pub ins: HashSet<Register>,
    pub outs: Vec<Register>,
}

#[derive(new)]
pub struct DisplayableLivenessGraph<'a> {
    graph: &'a LivenessGraph<'a>,
    entry: &'a Label,
}

impl Display for LivenessInfo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} d={}, u={}, i={}, o={}", self.instr, DisplayableVec(&self.defs), DisplayableVec(&self.uses), DisplayableSet(&self.ins), DisplayableVec(&self.outs))
    }
}


impl DisplayableLivenessGraph<'_> {
    fn visit(&self, visited: &mut HashSet<Label>, label: &Label, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !visited.contains(label) {
            visited.insert(label.clone());
            let liveness_info = self.graph.infos.get(label).unwrap().clone();
            writeln!(f, "\t{}: {}", label, liveness_info)?;
            match liveness_info.instr {
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

impl Display for DisplayableLivenessGraph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut visited = HashSet::new();

        self.visit(&mut visited, &self.entry, f)
    }
}
