use std::collections::HashMap;
use derive_new::new;
use crate::coloring::structure::Coloring;
use crate::ertl::structure::Label;
use crate::ertl::structure::register::Register;
use crate::ltl::error::LtlError;
use crate::ltl::LtlResult;
use crate::ltl::structure::{Instr, Operand};
use crate::rtl::structure::Fresh;

#[derive(new)]
pub struct Context<'a> {
    pub coloring: Coloring,
    pub graph: HashMap<Label, Instr<'a>>,
}

impl<'a> Context<'a> {
    pub fn insert(&mut self, instr: Instr<'a>) -> Label {
        let label = Label::fresh();
        self.graph.insert(label.clone(), instr);
        label
    }

    pub fn insert_at_label(&mut self, label: Label, instr: Instr<'a>) {
        self.graph.insert(label, instr);
    }

    pub fn color(&self, reg: &Register) -> LtlResult<Operand> {
        match reg {
            Register::Pseudo(reg) => {
                self.coloring.colors.get(reg).ok_or(LtlError::MissingRegisterColor(reg.clone())).cloned()
            }
            Register::Physical(reg) => {
                Ok(Operand::Register(reg.clone()))
            }
        }
    }
}