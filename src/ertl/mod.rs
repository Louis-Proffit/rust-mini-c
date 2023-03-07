pub mod structure;
pub mod error;

use std::collections::{HashMap, HashSet};
use itertools::enumerate;
use crate::common::Value;
use crate::ertl::error::ErtlError;
use crate::ertl::structure::{Graph, File, Fun, Instr, Label};
use crate::ertl::structure::register::{CALLEE_SAVED, PARAMETERS, PhysicalRegister, Register, RESULT};
use crate::rtl::structure as rtl;
use crate::rtl::structure::{Fresh, Mbinop, Munop};

pub type ErtlResult<T> = Result<T, ErtlError>;

pub fn ertl_file<'a>(file: &rtl::File<'a>) -> ErtlResult<File<'a>> {
    let mut funs = HashMap::new();

    for (name, fun) in &file.funs {
        funs.insert(name.clone(), ertl_fun(fun)?);
    }

    Ok(File::new(funs))
}

fn ertl_fun<'a>(fun: &rtl::Fun<'a>) -> ErtlResult<Fun<'a>> {
    let mut body = Graph::new(HashMap::new());

    let locals = HashSet::from_iter(fun.locals.values().map(|r| r.clone().into()));

    for (label, instr) in &fun.graph.instrs {
        ertl_instr(&mut body, label, instr)?;
    }

    let callee_saved_regs = CALLEE_SAVED
        .iter()
        .map(|x| { (x.clone(), Register::Pseudo(rtl::register::PseudoRegister::fresh())) })
        .collect::<Vec<(PhysicalRegister, Register)>>();

    let return_lbl = body.insert(Instr::EReturn);
    let delete_frame_lbl = body.insert(Instr::EDeleteFrame(return_lbl));

    let mut callee_restore_lbl = delete_frame_lbl;

    for (callee_saved, pseudo) in &callee_saved_regs {
        callee_restore_lbl = body.insert(Instr::EMBinop(Mbinop::MMov, pseudo.clone(), Register::Physical(callee_saved.clone()), callee_restore_lbl.clone()));
    }
    body.insert_at_label(&fun.exit, Instr::EMBinop(Mbinop::MMov, fun.result.clone().into(), Register::Physical(PhysicalRegister::Rax), callee_restore_lbl));

    let mut fetch_arg_lbl = (&fun.entry).clone();

    let args_count = fun.arguments.len() as u8;

    for (index, arg_reg) in enumerate(&fun.arguments).rev() {
        if index >= 6 {
            fetch_arg_lbl = body.insert(Instr::EGetParam((8 * (index - 6)) as u8, arg_reg.clone().into(), fetch_arg_lbl.clone()));
        } else {
            fetch_arg_lbl = body.insert(Instr::EMBinop(Mbinop::MMov, Register::Physical(PARAMETERS[index].clone()), arg_reg.clone().into(), fetch_arg_lbl));
        }
    }

    let mut callee_save_lbl = fetch_arg_lbl;

    for (callee_saved, pseudo) in &callee_saved_regs {
        callee_save_lbl = body.insert(Instr::EMBinop(Mbinop::MMov, Register::Physical(callee_saved.clone()), pseudo.clone(), callee_save_lbl.clone()));
    }

    let alloc_frame_lbl = body.insert(Instr::EAllocFrame(callee_save_lbl));

    Ok(Fun::new(
        fun.name,
        args_count,
        locals,
        alloc_frame_lbl,
        body,
    ))
}

fn ertl_instr<'a>(graph: &mut Graph<'a>, label: &Label, instr: &rtl::Instr<'a>) -> ErtlResult<()> {
    match instr {
        rtl::Instr::EConst(x, r, l) => graph.insert_at_label(label, Instr::EConst(x.clone().into(), r.clone().into(), l.clone())),
        rtl::Instr::ELoad(x, o, r, l) => graph.insert_at_label(label, Instr::ELoad(x.clone().into(), *o, r.clone().into(), l.clone())),
        rtl::Instr::EStore(x, r, o, l) => graph.insert_at_label(label, Instr::EStore(x.clone().into(), r.clone().into(), *o, l.clone())),
        rtl::Instr::EMUnop(op, r, l) => graph.insert_at_label(label, Instr::EMUnop(op.clone(), r.clone().into(), l.clone())),
        rtl::Instr::EMBinop(Mbinop::MDiv, r1, r2, l) => {
            let post_mov_label = graph.insert(Instr::EMBinop(Mbinop::MMov, Register::Physical(PhysicalRegister::Rax), r1.clone().into(), l.clone()));
            let div_lbl = graph.insert(Instr::EMBinop(Mbinop::MDiv, r1.clone().into(), Register::Physical(PhysicalRegister::Rax), post_mov_label));
            graph.insert_at_label(label, Instr::EMBinop(Mbinop::MMov, r2.clone().into(), Register::Physical(PhysicalRegister::Rax), div_lbl))
        }
        rtl::Instr::EMBinop(op, r1, r2, l) => graph.insert_at_label(label, Instr::EMBinop(op.clone(), r1.clone().into(), r2.clone().into(), l.clone())),
        rtl::Instr::EMuBranch(op, r, l1, l2) => graph.insert_at_label(label, Instr::EMuBranch(op.clone(), r.clone().into(), l1.clone(), l2.clone())),
        rtl::Instr::EMbBranch(op, r1, r2, l1, l2) => graph.insert_at_label(label, Instr::EMbBranch(op.clone(), r1.clone().into(), r2.clone().into(), l1.clone(), l2.clone())),
        rtl::Instr::ECall(r, name, args, l) => {
            let args_count = args.len() as u8;
            let args_on_stack = if args_count <= 6 { 0 } else { args_count - 6 };
            let args_in_registers = args_count - args_on_stack;

            let rsp_operation_lbl = graph.insert(Instr::EMUnop(Munop::Maddi((args_on_stack * 8) as Value), Register::Physical(PhysicalRegister::Rsp), l.clone()));
            let result_lbl = graph.insert(Instr::EMBinop(Mbinop::MMov, Register::Physical(RESULT), r.clone().into(), rsp_operation_lbl));

            if args_count > 0 {
                let call_lbl = graph.insert(Instr::ECall(name.clone(), args_in_registers, result_lbl.clone()));

                let mut next_arg_label = call_lbl;

                for (index, arg) in enumerate(args).rev() {
                    if index >= 6 {
                        next_arg_label = graph.insert(Instr::EPushParam(arg.clone().into(), next_arg_label.clone()));
                    } else if index > 0 {
                        next_arg_label = graph.insert(Instr::EMBinop(Mbinop::MMov, arg.clone().into(), Register::Physical(PARAMETERS[index].clone()), next_arg_label.clone()));
                    } else {
                        graph.insert_at_label(label, Instr::EMBinop(Mbinop::MMov, arg.clone().into(), Register::Physical(PARAMETERS[0].clone()), next_arg_label.clone()));
                        return Ok(());
                    }
                }
            }
            graph.insert_at_label(label, Instr::ECall(name.clone(), args_in_registers, result_lbl))
        }
        rtl::Instr::EGoto(l) => graph.insert_at_label(label, Instr::EGoto(l.clone()))
    }
    Ok(())
}