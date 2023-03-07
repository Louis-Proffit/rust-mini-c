use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::ltl::structure::Operand;
use crate::rtl::structure::register::PseudoRegister;

pub type Color = Operand;

#[derive(Debug, new)]
pub struct Coloring {
    pub colors: HashMap<PseudoRegister, Color>,
    pub count_on_stack: u16,
}

impl Display for Coloring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Emplacements de pile : {}", self.count_on_stack)?;

        for (reg, color) in &self.colors {
            writeln!(f,"{} -> {}", reg, color)?;
        }

        Ok(())
    }
}