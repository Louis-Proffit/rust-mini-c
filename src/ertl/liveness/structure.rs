use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::ertl::structure::{Instr, Label};
use crate::ertl::structure::register::Register;
use crate::utils::DisplayableSet;

#[derive(Debug, new)]
pub struct LivenessInfo<'a> {
    instr: Instr<'a>,
    succ: Vec<Label>,
    pred: HashSet<Label>,
    defs: Vec<Register>,
    uses: Vec<Register>,
    ins: Vec<Register>,
    outs: Vec<Register>,
}

pub struct LivenessGraph<'a> {
    pub(crate) infos: HashMap<Label, LivenessInfo<'a>>,
}

impl Display for LivenessInfo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} d={}, u={}, i={}, o={}", self.instr, DisplayableSet(&self.defs), DisplayableSet(&self.uses), DisplayableSet(&self.ins), DisplayableSet(&self.outs))
    }
}