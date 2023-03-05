pub mod structure;
pub mod error;

use std::collections::{HashMap, HashSet};
use crate::ertl::liveness::error::LivenessError;
use crate::ertl::liveness::structure::{LivenessGraph, LivenessInfo};
use crate::ertl::structure::{Graph, Instr, Label, Mbinop};
use crate::ertl::structure::register::{CALLEE_SAVED, CALLER_SAVED, PARAMETERS, Register};

pub type LivenessResult<T> = Result<T, LivenessError>;

pub fn liveness(graph: &Graph) -> LivenessResult<LivenessGraph> {
    let mut succ_map = HashMap::new();

    for (label, instr) in &graph.instrs {
        succ_map.insert(label.clone(), succ(instr))
    }

    let mut pred_map = HashMap::new();

    for (label, _) in &succ_map {
        pred_map.insert(label, HashSet::new());
    }

    for (label, succ) in &succ_map {
        for succ in succ {
            pred_map.get_mut(succ).unwrap().insert(label.clone())
        }
    }

    let mut def_use_map = HashMap::new();

    for (label, instr) in &graph.instrs {
        def_use_map.insert(label.clone(), def_use(instr));
    }


    let mut ins_map = HashMap::new();
    let mut outs_map = HashMap::new();

    // TODO fill ins et outs

    let mut infos = HashMap::new();

    for (label, instr) in graph.instrs {
        let (def, _use) = def_use_map.remove(&label).unwrap();

        infos.insert(label.clone(), LivenessInfo::new(
            instr.clone(),
            succ_map.remove(&label).unwrap(),
            pred_map.remove(&label).unwrap(),
            def,
            _use,
            ins_map.remove(&label).unwrap(),
            outs_map.remove(&label).unwrap(),
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
            assert_eq!(rd, Register::Rax);
            (vec![Register::Rax, Register::Rdx], vec![Register::Rax, Register::Rdx, rs.clone()])
        }
        Instr::EMBinop(_, rs, rd, _) => (vec![rd.clone()], vec![rs.clone(), rd.clone()]),
        Instr::EStore(r1, r2, _, _)
        | Instr::EMbBranch(_, r1, r2, _, _) => (vec![], vec![r1.clone(), r2.clone()]),
        Instr::ECall(_, n, _) => {
            (Vec::from(CALLER_SAVED), Vec::from(PARAMETERS).drain(..n).collect())
        }
        Instr::EGoto(_)
        | Instr::EAllocFrame(_)
        | Instr::EDeleteFrame(_) => (vec![], vec![]),
        Instr::EReturn => {
            let mut used = vec![Register::Rax];
            used.extend(CALLEE_SAVED);
            (vec![], used)
        }
    }
}
