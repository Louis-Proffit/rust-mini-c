use std::collections::HashSet;
use crate::ertl::structure::{Label, Mbinop, MuBranch, Munop};
use crate::ertl::structure::register::TMP_1;
use crate::linearise::context::Context;
use crate::linearise::error::LinearisingError;
use crate::linearise::x86_64::{Asm, AsmNode, Program, Section, Size, SizedPhysicalRegister, X86Operand};
use crate::ltl::structure::{File, Graph, Instr, Operand};

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
            if context.labels.contains(l) {
                context.need_label(l.clone());
                context.emit_at_label(label.clone(), AsmNode::Jmp(l.clone()));
                Ok(())
            } else {
                context.emit(AsmNode::Label(label.clone()));
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
                _ => {
                    let (r_or_reg, label) = match r {
                        Operand::Reg(r) => (r, Some(label)),
                        Operand::Spilled(_) => {
                            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r.clone().into(), TMP_1.into()));
                            (&TMP_1, None)
                        }
                    };
                    let l = match op {
                        Munop::Msetei(v) => {
                            context.emit_at_label_or(label.cloned(), AsmNode::Cmp(Size::Q, r_or_reg.clone().into(), (*v).into()));
                            context.emit(AsmNode::Sete((r_or_reg.clone(), Size::B).into()));
                            l
                        }
                        Munop::Msetnei(v) => {
                            context.emit_at_label_or(label.cloned(), AsmNode::Cmp(Size::Q, r_or_reg.clone().into(), (*v).into()));
                            context.emit(AsmNode::Setne((r_or_reg.clone(), Size::B).into()));
                            l
                        }
                        _ => unreachable!()
                    };
                    match r {
                        Operand::Reg(_) => {}
                        Operand::Spilled(_) => {
                            context.emit(AsmNode::Mov(Size::Q, r_or_reg.clone().into(), r.clone().into()));
                        }
                    };
                    lin(context, graph, l)
                }
            }
        }
        Instr::EMBinop(op, r1, r2, l) => {
            let r1 = match (r1, r2) {
                (Operand::Spilled(_), Operand::Spilled(_)) => {
                    context.emit(AsmNode::Mov(Size::Q, r1.clone().into(), X86Operand::Register(TMP_1)));
                    X86Operand::Register(TMP_1)
                }
                _ => {
                    r1.clone().into()
                }
            };
            match op {
                Mbinop::MMov => {
                    context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r1, r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MAdd => {
                    context.emit_at_label(label.clone(), AsmNode::Add(Size::Q, r1, r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MSub => {
                    context.emit_at_label(label.clone(), AsmNode::Sub(Size::Q, r1, r2.clone().into()));
                    lin(context, graph, l)
                }
                Mbinop::MMul => {
                    match r2 {
                        Operand::Reg(r) => {
                            context.emit_at_label(label.clone(), AsmNode::Imul(Size::Q, r1, r2.clone().into()));
                            lin(context, graph, l)
                        }
                        Operand::Spilled(_) => {
                            let r2: X86Operand = r2.clone().into();
                            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r2.clone(), TMP_1.into()));
                            context.emit(AsmNode::Imul(Size::Q, r1, TMP_1.into()));
                            context.emit(AsmNode::Mov(Size::Q, TMP_1.into(), r2));
                            lin(context, graph, l)
                        }
                    }
                }
                Mbinop::MDiv => {
                    context.emit_at_label(label.clone(), AsmNode::Cqto);
                    context.emit(AsmNode::IDivq(r2.clone().into()));
                    lin(context, graph, l)
                }
                _ => {
                    let (r_or_reg, label) = match r2 {
                        Operand::Reg(r) => (r, Some(label)),
                        Operand::Spilled(_) => {
                            context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r2.clone().into(), TMP_1.into()));
                            (&TMP_1, None)
                        }
                    };
                    context.emit_at_label_or(label.cloned(), AsmNode::Cmp(Size::Q, r1, r_or_reg.clone().into()));
                    let size_r_or_reg: SizedPhysicalRegister = (r_or_reg.clone(), Size::B).into();
                    let l = match op {
                        Mbinop::MSete => {
                            context.emit(AsmNode::Sete(size_r_or_reg));
                            l
                        }
                        Mbinop::MSetne => {
                            context.emit(AsmNode::Setne(size_r_or_reg));
                            l
                        }
                        Mbinop::Msetl => {
                            context.emit(AsmNode::Setl(size_r_or_reg));
                            l
                        }
                        Mbinop::Msetle => {
                            context.emit(AsmNode::Setle(size_r_or_reg));
                            l
                        }
                        Mbinop::Msetg => {
                            context.emit(AsmNode::Setg(size_r_or_reg));
                            l
                        }
                        Mbinop::Msetge => {
                            context.emit(AsmNode::Setge(size_r_or_reg));
                            l
                        }
                        _ => unreachable!()
                    };
                    match r2 {
                        Operand::Reg(_) => {}
                        Operand::Spilled(_) => {
                            context.emit(AsmNode::Mov(Size::Q, r_or_reg.clone().into(), r2.clone().into()));
                        }
                    }
                    lin(context, graph, l)
                }
            }
        }
        Instr::EMuBranch(op, r, l1, l2) => {
            match r {
                Operand::Reg(_) => {
                    context.emit_at_label(label.clone(), AsmNode::Test(Size::Q, r.clone().into(), r.clone().into()));
                }
                Operand::Spilled(_) => {
                    context.emit_at_label(label.clone(), AsmNode::Mov(Size::Q, r.clone().into(), TMP_1.into()));
                    context.emit(AsmNode::Test(Size::Q, TMP_1.into(), TMP_1.into()));
                }
            }
            if !context.visited.contains(l2) {
                match op {
                    MuBranch::MJz => {
                        context.emit_at_label(label.clone(), AsmNode::Jz(l1.clone()));
                        context.need_label(l1.clone());
                        lin(context, graph, l2)?;
                        lin(context, graph, l1)
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jnz(l1.clone()));
                        context.need_label(l1.clone());
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
                        context.need_label(l2.clone());
                        lin(context, graph, l1)?;
                        lin(context, graph, l2)
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jz(l2.clone()));
                        context.need_label(l2.clone());
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
                        context.need_label(l1.clone());
                        context.need_label(l2.clone());
                        Ok(())
                    }
                    MuBranch::MJnz => {
                        context.emit_at_label(label.clone(), AsmNode::Jnz(l1.clone()));
                        context.emit_at_label(label.clone(), AsmNode::Jmp(l2.clone()));
                        context.need_label(l1.clone());
                        context.need_label(l2.clone());
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