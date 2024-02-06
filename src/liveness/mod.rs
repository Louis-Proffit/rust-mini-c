use std::collections::{BTreeSet, HashMap, HashSet};
use crate::ertl::structure::{Graph, Instr, Label, Mbinop};
use crate::ertl::structure::register::{CALLER_SAVED, PARAMETERS, PhysicalRegister, Register};
use crate::liveness::error::LivenessError;
use crate::liveness::structure::{LivenessGraph, LivenessInfo};

pub mod structure;
pub mod error;


pub type LivenessResult<T> = Result<T, LivenessError>;

pub fn liveness_graph<'a>(graph: &'a Graph<'a>) -> LivenessResult<LivenessGraph<'a>> {
    let mut succ_map = HashMap::new();

    for (label, instr) in &graph.instrs {
        succ_map.insert(label.clone(), succ(instr));
    }

    let mut pred_map = HashMap::new();

    for (label, _) in &graph.instrs {
        pred_map.insert(label.clone(), HashSet::new());
    }

    for (label, succ) in &succ_map {
        for succ in succ {
            pred_map.get_mut(succ).unwrap().insert(label.clone());
        }
    }

    let mut def_use_map = HashMap::new();

    for (label, instr) in &graph.instrs {
        def_use_map.insert(label.clone(), def_use(instr));
    }

    let mut outs_map: HashMap<Label, Vec<Register>> = HashMap::new();

    let mut ins_map: HashMap<Label, HashSet<Register>> = HashMap::new();
    for (label, _) in &graph.instrs {
        ins_map.insert(label.clone(), HashSet::new());
    }

    let mut to_handle: BTreeSet<&Label> = graph.instrs.keys().collect();

    while let Some(label) = to_handle.pop_first() {
        let old_in = ins_map.get(label).unwrap();

        let mut new_out = Vec::new();
        for succ in succ_map.get(label).unwrap() {
            for label_in in ins_map.get(succ).unwrap() {
                new_out.push(label_in.clone());
            }
        }

        let (def, used) = def_use_map.get(label).unwrap();
        let mut new_in = HashSet::new();

        for out in &new_out {
            new_in.insert(out.clone());
        }

        for def in def {
            new_in.remove(def);
        }

        for used in used {
            new_in.insert(used.clone());
        }


        if !new_in.difference(old_in).next().is_none() {
            for pred in pred_map.get(label).unwrap() {
                to_handle.insert(pred);
            }
        }

        outs_map.insert(label.clone(), new_out);
        ins_map.insert(label.clone(), new_in);
    }

    let mut infos = HashMap::new();

    for (label, instr) in &graph.instrs {
        let (def, uses) = def_use_map.remove(label).unwrap();

        infos.insert(label.clone(), LivenessInfo::new(
            instr,
            succ_map.remove(label).unwrap(),
            pred_map.remove(label).unwrap(),
            def,
            uses,
            ins_map.remove(label).unwrap(),
            outs_map.remove(label).unwrap(),
        ));
    }

    let graph = LivenessGraph { infos };

    Ok(graph)
}

fn succ(instr: &Instr) -> Vec<Label> {
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
        | Instr::EPushParam(_, l) => vec![l.clone()],
        Instr::EMuBranch(_, _, l1, l2)
        | Instr::EMbBranch(_, _, _, l1, l2) => vec![l1.clone(), l2.clone()],
        Instr::EReturn => vec![]
    }
}

pub fn def_use(instr: &Instr) -> (Vec<Register>, Vec<Register>) {
    match instr {
        Instr::EConst(_, r, _)
        | Instr::EGetParam(_, r, _) => (vec![r.clone()], vec![]),
        Instr::EMuBranch(_, r, _, _)
        | Instr::EPushParam(r, _) => (vec![], vec![r.clone()]),
        Instr::EMUnop(_, r, _) => (vec![r.clone()], vec![r.clone()]),
        Instr::EMBinop(Mbinop::MMov, rs, rd, _)
        | Instr::ELoad(rs, _, rd, _) => (vec![rd.clone()], vec![rs.clone()]),
        Instr::EMBinop(Mbinop::MDiv, rs, rd, _) => {
            assert_eq!(rd, &Register::Physical(PhysicalRegister::Rax));
            (vec![Register::Physical(PhysicalRegister::Rax), Register::Physical(PhysicalRegister::Rdx)], vec![Register::Physical(PhysicalRegister::Rax), Register::Physical(PhysicalRegister::Rdx), rs.clone()])
        }
        Instr::EMBinop(_, rs, rd, _) => (vec![rd.clone()], vec![rs.clone(), rd.clone()]),
        Instr::EStore(r1, r2, _, _)
        | Instr::EMbBranch(_, r1, r2, _, _) => (vec![], vec![r1.clone(), r2.clone()]),
        Instr::ECall(_, n, _) => {
            let def = CALLER_SAVED.iter()
                .map(|r| Register::Physical(r.clone()))
                .collect();
            let used = PARAMETERS
                .iter()
                .map(|r| Register::Physical(r.clone()))
                .collect::<Vec<Register>>()
                .drain(..(*n as usize))
                .collect();
            (def, used)
        }
        Instr::EGoto(_)
        | Instr::EAllocFrame(_)
        | Instr::EDeleteFrame(_) => (vec![], vec![]),
        Instr::EReturn => {
            let mut used = CALLER_SAVED
                .into_iter()
                .map(|r| { Register::Physical(r) })
                .collect::<Vec<Register>>();

            used.push(Register::Physical(PhysicalRegister::Rax));
            (vec![], used)
        }
    }
}
