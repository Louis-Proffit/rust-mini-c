use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::ertl::structure::register::Register;
use crate::utils::DisplayableSet;

#[derive(Debug, new)]
pub struct RegisterArc {
    pub prefs: HashSet<Register>,
    pub intfs: HashSet<Register>,
}

#[derive(Debug, new)]
pub struct InterferenceGraph {
    pub arcs: HashMap<Register, RegisterArc>,
}

impl InterferenceGraph {
    pub fn pref(&mut self, r1: &Register, r2: &Register) {
        self.pref_oriented(r1, r2);
        self.pref_oriented(r2, r1);
    }

    pub fn intf(&mut self, r1: &Register, r2: &Register) {
        self.intf_oriented(r1, r2);
        self.intf_oriented(r2, r1);
    }


    pub fn remove_pref_when_intf(&mut self) {
        for (_reg, arc) in &mut self.arcs {
            for intf in &arc.intfs {
                arc.prefs.remove(intf);
            }
        }
    }

    fn pref_oriented(&mut self, r1: &Register, r2: &Register) {
        match self.arcs.get_mut(r1) {
            None => {
                let arc = RegisterArc::new(HashSet::from([r2.clone()]), HashSet::new());
                self.arcs.insert(r1.clone(), arc);
            }
            Some(arc) => {
                arc.prefs.insert(r2.clone());
            }
        }
    }

    fn intf_oriented(&mut self, r1: &Register, r2: &Register) {
        match self.arcs.get_mut(r1) {
            None => {
                let arc = RegisterArc::new(HashSet::new(), HashSet::from([r2.clone()]));
                self.arcs.insert(r1.clone(), arc);
            }
            Some(arc) => {
                arc.intfs.insert(r2.clone());
            }
        }
    }
}

impl Display for InterferenceGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (reg, arc) in &self.arcs {
            writeln!(f, "{}: prefs:{}, intfs:{}", reg, DisplayableSet(&arc.prefs), DisplayableSet(&arc.intfs))?;
        }
        Ok(())
    }
}