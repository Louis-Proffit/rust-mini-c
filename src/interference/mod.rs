use std::collections::HashMap;
use crate::ertl::structure::{Instr, Mbinop};
use crate::interference::structure::InterferenceGraph;
use crate::liveness::structure::LivenessGraph;

pub mod structure;

pub type InterferenceError = ();

pub type LivenessResult<T> = Result<T, InterferenceError>;

pub fn interference_graph(graph: &LivenessGraph) -> LivenessResult<InterferenceGraph> {
    let mut result = InterferenceGraph::new(HashMap::new());

    for (_, liveness_info) in &graph.infos {
        match liveness_info.instr {
            Instr::EMBinop(Mbinop::MMov, r1, r2, _) if r1 != r2 => result.pref(r1, r2),
            _ => {}
        }
    }

    for (_, liveness_info) in &graph.infos {
        for def in &liveness_info.defs {
            for out in &liveness_info.outs {
                if def != out {
                    result.intf(def, out)
                }
            }
        }
    }

    result.remove_pref_when_intf();

    Ok(result)
}