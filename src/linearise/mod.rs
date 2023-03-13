use std::collections::HashSet;
use crate::ertl::structure::{Label, Mbinop, MuBranch, Munop};
use crate::linearise::context::Context;
use crate::linearise::error::LinearisingError;
use crate::linearise::x86_64::{Asm, AsmNode, Program, Section, Size, X86Operand};
use crate::ltl::structure::{File, Graph, Instr};

pub mod x86_64;
pub mod error;
mod context;

pub type LinearisingResult<T> = Result<T, LinearisingError>;

pub fn linearise<'a>(file: &File<'a>) -> LinearisingResult<Program<'a>> {
    Ok(Program::new(vec![
        Section::Text(text(file)?),
        Section::Data(data(file)?),
    ]))
}

fn text<'a>(file: &File<'a>) -> LinearisingResult<Asm<'a>> {
    let mut context = Context::new(Asm::new(Vec::new()), HashSet::new(), HashSet::new());

    for (name, fun) in &file.funs {
        context.emit(AsmNode::DeclFun(name.clone()));
        lin(&mut context, &fun.body, &fun.entry)?;
    }

    let mut nodes = vec![AsmNode::Globl("main")];

    nodes.extend(context.code.nodes
        .into_iter()
        .filter(|l| {
            match l {
                AsmNode::Label(l) => {
                    context.labels.contains(l)
                }
                _ => true
            }
        }));

    Ok(Asm::new(nodes))
}

fn lin<'a>(context: &mut Context<'a>, graph: &Graph<'a>, label: &Label) -> LinearisingResult<()> {
    if !context.visited.contains(label) {
        context.visited.insert(label.clone());
        instr(context, graph, label, graph.instrs.get(label).expect("Instr not found for lbl"))
    } else {
        context.need_label(label.clone());
        context.emit(AsmNode::Jmp(label.clone()));
        Ok(())
    }
}

fn instr<'a>(context: &mut Context<'a>, graph: &Graph<'a>, label: &Label, instr: &Instr<'a>) -> LinearisingResult<()> {
    match instr {
        Instr::ELoad(from, o, dest, l) => {
            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, X86Operand::Offset(*o, from.clone()), dest.clone().into()));
            lin(context, graph, l)
        }
        Instr::EStore(value, addr, o, l) => {
            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, value.clone().into(), X86Operand::Offset(*o, addr.clone())));
            lin(context, graph, l)
        }
        Instr::EGoto(l) => {
            if context.visited.contains(l) {
                context.emit_at_label(label.clone(), AsmNode::Jmp(l.clone()));
                Ok(())
            } else {
                context.emit_at_label(label.clone(), AsmNode::Label(label.clone()));
                lin(context, graph, l)
            }
        }
        Instr::EReturn => {
            context.emit_at_label(label.clone(), AsmNode::Ret);
            Ok(())
        }
        Instr::EConst(v, o, l) => {
            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, (*v).into(), o.clone().into()));
            lin(context, graph, l)
        }
        Instr::EMunop(op, r, l) => {
            match op {
                Munop::Maddi(v) => {
                    context.emit_at_label(label.clone(), AsmNode::Add(Size::Q, (*v).into(), r.clone().into()));
                    lin(context, graph, l)
                }
                Munop::Msetei(v) => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r.clone().into(), (*v).into()));
                    context.emit(AsmNode::Sete(r.clone().into()));
                    lin(context, graph, l)
                }
                Munop::Msetnei(v) => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r.clone().into(), (*v).into()));
                    context.emit(AsmNode::Setne(r.clone().into()));
                    lin(context, graph, l)
                }
            }
        }
        Instr::EMBinop(op, r1, r2, l) => {
            match op {
                Mbinop::MMov => {
                    context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r1.clone().into(), r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MAdd => {
                    context.emit_at_label(label.clone(), AsmNode::Add(Size::Q, r1.clone().into(), r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MSub => {
                    context.emit_at_label(label.clone(), AsmNode::Sub(Size::Q, r1.clone().into(), r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MMul => {
                    context.emit_at_label(label.clone(), AsmNode::Imul(Size::Q, r1.clone().into(), r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MDiv => {
                    context.emit_at_label(label.clone(), AsmNode::Cqto);
                    context.emit(AsmNode::IDivq(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MSete => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Sete(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MSetne => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Setne(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::Msetl => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Setl(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::Msetle => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Setle(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::Msetg => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Setg(r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::Msetge => {
                    context.emit_at_label(label.clone(), AsmNode::Cmp(Size::Q, r1.clone().into(), r2.clone().into()));
                    context.emit(AsmNode::Setge(r2.clone().into()));
                    lin(context, graph, l)
                }
            }
        }
        Instr::EMuBranch(op, r, l1, l2) => {
            context.emit_at_label(label.clone(), AsmNode::Test(Size::Q, r.clone().into(), r.clone().into()));
            if !context.visited.contains(l2) {
                match op {
                    MuBranch::MJz => {
                        context.emit_at_label(label.clone(), AsmNode::Jz(l1.clone()));
                        lin(context, graph, l2)?;
                        lin(context, graph, l1)
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jnz(l1.clone()));
                        lin(context, graph, l2)?;
                        lin(context, graph, l1)
                    }
                    MuBranch::MJlei(v) => todo!(),
                    MuBranch::MJgi(v) => todo!(),
                }
            } else if !context.visited.contains(l1) {
                match op {
                    MuBranch::MJz => {
                        context.emit_at_label(label.clone(), AsmNode::Jnz(l2.clone()));
                        lin(context, graph, l1)?;
                        lin(context, graph, l2)
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jz(l2.clone()));
                        lin(context, graph, l1)?;
                        lin(context, graph, l2)
                    }
                    MuBranch::MJlei(_) => todo!(),
                    MuBranch::MJgi(_) => todo!()
                }
            } else {
                match op {
                    MuBranch::MJz => {
                        context.emit_at_label(label.clone(), AsmNode::Jz(l1.clone()));
                        context.emit_at_label(label.clone(), AsmNode::Jmp(l2.clone()));
                        Ok(())
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jnz(l1.clone()));
                        context.emit_at_label(label.clone(), AsmNode::Jmp(l2.clone()));
                        Ok(())
                    }
                    MuBranch::MJlei(_) => todo!(),
                    MuBranch::MJgi(_) => todo!()
                }
            }
        }
        Instr::EMbBranch(_, _, _, _, _) => todo!(),
        Instr::EPush(o, l) => {
            context.emit_at_label(label.clone(), AsmNode::Pushq(o.clone().into()));
            lin(context, graph, l)
        }
        Instr::ECall(i, l) => {
            context.emit_at_label(label.clone(), AsmNode::Call(i.clone()));
            lin(context, graph, l)
        }
        Instr::EPop(o, l) => {
            context.emit_at_label(label.clone(), AsmNode::Popq(o.clone().into()));
            lin(context, graph, l)
        }
    }
}

fn data<'a>(_file: &File<'a>) -> LinearisingResult<Asm<'a>> {
    Ok(Asm::new(Vec::new()))
}