pub mod structure;
pub mod error;
mod context;

use std::collections::HashMap;
use crate::coloring::color_graph;
use crate::common::Value;
use crate::ertl::structure as ertl;
use crate::ertl::structure::{Label, Mbinop};
use crate::ertl::structure::register::{PhysicalRegister, TMP_1};
use crate::interference::interference_graph;
use crate::liveness::liveness_graph;
use crate::ltl::context::Context;
use crate::ltl::error::LtlError;
use crate::ltl::structure::{File, Fun, Graph, Instr, Operand};
use crate::rtl::structure::Munop;


pub type LtlResult<T> = Result<T, LtlError>;

pub fn ltl_file<'a>(file: &ertl::File<'a>) -> LtlResult<File<'a>> {
    let mut funs = HashMap::new();

    for (name, fun) in &file.funs {
        funs.insert(name.clone(), ltl_fun(fun)?);
    }

    Ok(File::new(funs))
}

fn ltl_fun<'a>(fun: &ertl::Fun<'a>) -> LtlResult<Fun<'a>> {
    let liveness = liveness_graph(&fun.body).map_err(|err| LtlError::LivenessError(err))?;
    let interference = interference_graph(&liveness).map_err(|err| LtlError::InterferenceError(err))?;
    let coloring = color_graph(&interference).map_err(|err| LtlError::ColoringError(err))?;

    let mut context = Context::new(
        fun.locals.len() as u16 * 8,
        coloring,
        HashMap::new(),
    );

    for (label, instr) in &fun.body.instrs {
        ltl_instr(&mut context, label, instr)?;
    }

    Ok(Fun::new(
        fun.name.clone(),
        fun.entry.clone(),
        Graph::new(context.graph),
    ))
}

fn ltl_instr<'a>(context: &mut Context<'a>, label: &Label, instr: &ertl::Instr<'a>) -> LtlResult<()> {
    match instr {
        ertl::Instr::EConst(c, r, l) => {
            context.insert_at_label(
                label.clone(),
                Instr::EConst(*c, context.color(r)?, l.clone()),
            );
            Ok(())
        }
        ertl::Instr::ELoad(addr, o, dest, l) => {
            match (context.color(addr)?, context.color(dest)?) {
                (Operand::Reg(r1), Operand::Reg(r2)) => {
                    context.insert_at_label(
                        label.clone(),
                        Instr::ELoad(r1, *o, r2, l.clone()),
                    );
                    Ok(())
                }
                _ => todo!()
            }
        }
        ertl::Instr::EStore(value, addr, o, l) => {
            match (context.color(value)?, context.color(addr)?) {
                (Operand::Reg(r1), Operand::Reg(r2)) => {
                    context.insert_at_label(
                        label.clone(),
                        Instr::EStore(r1, r2, *o, l.clone()),
                    );
                    Ok(())
                }
                _ => todo!()
            }
        }
        ertl::Instr::EMUnop(op, r, l) => {
            let color = context.color(r)?;
            context.insert_at_label(
                label.clone(),
                Instr::EMunop(op.clone(), color, l.clone()),
            );
            Ok(())
        }
        ertl::Instr::EMBinop(op, r1, r2, l) => {
            let color_1 = context.color(r1)?;
            let color_2 = context.color(r2)?;
            if *op == Mbinop::MMov && color_1 == color_2 {
                context.insert_at_label(
                    label.clone(),
                    Instr::EGoto(l.clone()),
                )
            } else {
                context.insert_at_label(
                    label.clone(),
                    Instr::EMBinop(op.clone(), color_1, color_2, l.clone()),
                );
            }
            Ok(())
        }
        ertl::Instr::EMuBranch(op, r, l1, l2) => {
            context.insert_at_label(
                label.clone(),
                Instr::EMuBranch(op.clone(), context.color(r)?, l1.clone(), l2.clone()),
            );
            Ok(())
        }
        ertl::Instr::EMbBranch(op, r1, r2, l1, l2) => {
            context.insert_at_label(
                label.clone(),
                Instr::EMbBranch(
                    op.clone(),
                    context.color(r1)?,
                    context.color(r2)?,
                    l1.clone(),
                    l2.clone()),
            );
            Ok(())
        }
        ertl::Instr::ECall(name, _frame_size, l) => {
            // TODO no frame size needed ?
            context.insert_at_label(
                label.clone(),
                Instr::ECall(name.clone(), l.clone()),
            );
            Ok(())
        }
        ertl::Instr::EGoto(l) => {
            context.insert_at_label(
                label.clone(),
                Instr::EGoto(l.clone()),
            );
            Ok(())
        }
        ertl::Instr::EAllocFrame(l) => {
            // TODO simplifier pour frame_size = 0
            if context.frame_size != 0 {
                let add_rsp_lbl = context.insert(
                    Instr::EMunop(Munop::Maddi(-(8 * context.frame_size as Value)), Operand::Reg(PhysicalRegister::Rsp), l.clone())
                );
                let mov_rsp_lbl = context.insert(
                    Instr::EMBinop(Mbinop::MMov, Operand::Reg(PhysicalRegister::Rsp), Operand::Reg(PhysicalRegister::Rbp), add_rsp_lbl)
                );
                context.insert_at_label(
                    label.clone(),
                    Instr::EPush(Operand::Reg(PhysicalRegister::Rbp), mov_rsp_lbl),
                );
            } else {
                context.insert_at_label(
                    label.clone(),
                    Instr::EGoto(l.clone()),
                );
            }
            Ok(())
        }
        ertl::Instr::EDeleteFrame(l) => {
            if context.frame_size != 0 {
                let pop_lbl = context.insert(
                    Instr::EPop(Operand::Reg(PhysicalRegister::Rbp), l.clone()),
                );
                context.insert_at_label(
                    label.clone(),
                    Instr::EMBinop(Mbinop::MMov, Operand::Reg(PhysicalRegister::Rbp), Operand::Reg(PhysicalRegister::Rsp), pop_lbl),
                );
            } else {
                context.insert_at_label(
                    label.clone(),
                    Instr::EGoto(l.clone()),
                );
            }
            Ok(())
        }
        ertl::Instr::EGetParam(index, dest, l) => {
            let color = context.color(dest)?;
            match color {
                Operand::Reg(r) => {
                    context.insert_at_label(
                        label.clone(),
                        Instr::ELoad(PhysicalRegister::Rbp, (index - 7) * 8 + 16, r, l.clone()),
                    );
                    Ok(())
                }
                Operand::Spilled(o) => {
                    let tmp = TMP_1;
                    context.insert_at_label(
                        label.clone(),
                        Instr::ELoad(PhysicalRegister::Rbp, (index - 7) * 8 + 16, tmp.clone(), l.clone()),
                    );
                    context.insert_at_label(
                        label.clone(),
                        Instr::EStore(tmp, PhysicalRegister::Rbp, o, l.clone()),
                    );
                    Ok(())
                }
            }
        }
        ertl::Instr::EPushParam(r, l) => {
            context.insert_at_label(
                label.clone(),
                Instr::EPush(context.color(r)?, l.clone()),
            );
            Ok(())
        }
        ertl::Instr::EReturn => {
            context.insert_at_label(label.clone(), Instr::EReturn);
            Ok(())
        }
    }
}