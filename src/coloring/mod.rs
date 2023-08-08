use std::collections::{BTreeSet, HashMap, HashSet};
use crate::coloring::structure::Coloring;
use crate::common::StackOffset;
use crate::ertl::structure::register::{ALLOCATABLE, PhysicalRegister, Register};
use crate::interference::structure::InterferenceGraph;
use crate::ltl::structure::Operand;
use crate::rtl::structure::register::PseudoRegister;

pub mod structure;

pub type ColoringError = ();

pub type ColoringResult<T> = Result<T, ColoringError>;

pub fn color_graph(graph: &InterferenceGraph) -> ColoringResult<Coloring> {
    let mut allocatable = init_allocatable(graph)?;
    let mut colors = HashMap::new();
    let mut count_on_stack = 0;

    let mut todo = BTreeSet::new();

    for (reg, _) in &graph.arcs {
        match reg {
            Register::Pseudo(reg) => { todo.insert(reg.clone()); }
            _ => {}
        }
    }

    while !todo.is_empty() {
        if let Some((reg, color)) = search_one_color_with_preference(
            &todo,
            &allocatable,
            graph,
            &colors,
        ).or_else(|| {
            search_one_color(
                &todo,
                &allocatable,
            )
        }).or_else(|| {
            search_pref_with_known_color(
                &todo,
                &allocatable,
                graph,
                &colors,
            )
        }).or_else(|| {
            search_any_color(
                &todo,
                &allocatable,
            )
        }) {
            colors.insert(reg.clone(), Operand::Register(color.clone()));
            todo.remove(&reg);
            for other in &graph.arcs.get(&Register::Pseudo(reg)).unwrap().intfs {
                match other {
                    Register::Pseudo(other) => {
                        allocatable.get_mut(other).unwrap().remove(&color);
                    }
                    Register::Physical(_) => {}
                }
            }
        } else {
            let spilled = todo.pop_first().unwrap();
            colors.insert(spilled, Operand::Spilled(count_on_stack as StackOffset));
            count_on_stack += 1
        }
    }

    Ok(Coloring::new(colors, count_on_stack))
}

fn search_one_color_with_preference(
    todo: &BTreeSet<PseudoRegister>,
    allocatable: &HashMap<PseudoRegister, HashSet<PhysicalRegister>>,
    interference_graph: &InterferenceGraph,
    current_allocation: &HashMap<PseudoRegister, Operand>,
) -> Option<(PseudoRegister, PhysicalRegister)> {
    for reg in todo {
        let allocatable_colors = allocatable.get(reg).unwrap();
        if allocatable_colors.len() == 1 {
            for color in allocatable_colors {
                for pref in &interference_graph.arcs.get(&Register::Pseudo(reg.clone())).unwrap().prefs {
                    match pref {
                        Register::Pseudo(pref) => {
                            if current_allocation.contains_key(pref) {
                                match current_allocation.get(pref).unwrap() {
                                    Operand::Register(pref_color) if pref_color == color => {
                                        return Some((reg.clone(), color.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Register::Physical(pref) => {
                            if pref == color {
                                return Some((reg.clone(), color.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn search_one_color(
    todo: &BTreeSet<PseudoRegister>,
    allocatable: &HashMap<PseudoRegister, HashSet<PhysicalRegister>>,
) -> Option<(PseudoRegister, PhysicalRegister)> {
    for reg in todo {
        let colors = allocatable.get(reg).unwrap();
        if colors.len() == 1 {
            for color in colors {
                return Some((reg.clone(), color.clone()));
            }
        }
    }
    None
}

fn search_pref_with_known_color(
    todo: &BTreeSet<PseudoRegister>,
    allocatable: &HashMap<PseudoRegister, HashSet<PhysicalRegister>>,
    interference_graph: &InterferenceGraph,
    current_allocation: &HashMap<PseudoRegister, Operand>,
) -> Option<(PseudoRegister, PhysicalRegister)> {
    for reg in todo {
        for pref in &interference_graph.arcs.get(&Register::Pseudo(reg.clone())).unwrap().prefs {
            match pref {
                Register::Pseudo(pref) => {
                    match current_allocation.get(pref) {
                        Some(Operand::Register(color)) => {
                            if allocatable.get(reg).unwrap().contains(color) {
                                return Some((reg.clone(), color.clone()));
                            }
                        }
                        _ => {}
                    }
                }
                Register::Physical(pref) => {
                    if allocatable.get(reg).unwrap().contains(pref) {
                        return Some((reg.clone(), pref.clone()));
                    }
                }
            }
        }
    }
    None
}

fn search_any_color(
    todo: &BTreeSet<PseudoRegister>,
    allocatable: &HashMap<PseudoRegister, HashSet<PhysicalRegister>>,
) -> Option<(PseudoRegister, PhysicalRegister)> {
    for reg in todo {
        for color in allocatable.get(reg).unwrap() {
            return Some((reg.clone(), color.clone()));
        }
    }
    None
}

fn init_allocatable(graph: &InterferenceGraph) -> ColoringResult<HashMap<PseudoRegister, HashSet<PhysicalRegister>>> {
    let mut result = HashMap::new();

    for (reg, arc) in &graph.arcs {
        match reg {
            Register::Pseudo(reg) => {
                let mut allocatable = HashSet::from(ALLOCATABLE);
                for intf in &arc.intfs {
                    match intf {
                        Register::Pseudo(_) => {}
                        Register::Physical(physical_reg) => {
                            allocatable.remove(physical_reg);
                        }
                    }
                }
                result.insert(
                    reg.clone(),
                    allocatable,
                );
            }
            Register::Physical(_) => {}
        }
    }

    Ok(result)
}