mod default;
mod context;
mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;
use crate::common::{Ident, MAIN, MALLOC, PUTCHAR, Stdout, Value};
use crate::common::bool::{Bool, ToCBool};
use crate::rtl::interpreter::context::Context;
use crate::rtl::interpreter::default::malloc::Malloc;
use crate::rtl::interpreter::default::putchar::Putchar;
use crate::rtl::interpreter::error::RtlInterpreterError;
use crate::rtl::structure::{File, Fun, Instr, MbBranch, Mbinop, MuBranch, Munop};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

pub type RtlInterpreterResult<T> = Result<T, RtlInterpreterError>;

pub fn interp_rtl_file<'a>(file: &'a File) -> RtlInterpreterResult<Stdout> {
    let main = file.funs.get(MAIN).ok_or(RtlInterpreterError::FunctionDoesNotExist(String::from(MAIN)))?;

    let stdout = Rc::new(Stdout::new());
    let mut funs: HashMap<Ident, Rc<dyn RtlInterpFun + 'a>> = HashMap::new();

    for (name, fun) in &file.funs {
        funs.insert(name.clone(), Rc::new(fun));
    }

    funs.insert(PUTCHAR, Rc::new(Putchar::new()));
    funs.insert(MALLOC, Rc::new(Malloc::new()));

    let context = Context::new(
        stdout.clone(),
        Rc::new(funs),
        Rc::new(RefCell::new(HashMap::new())),
        Rc::new(RefCell::new(HashMap::new())),
    );

    main.interp_fun(&context)?;

    Ok(stdout.as_ref().clone())
}

pub trait RtlInterpFun<'a> {
    fn fun_result(&self) -> &Register;
    fn fun_arguments(&self) -> &Vec<Register>;
    fn interp_fun(&self, context: &Context<'a>) -> RtlInterpreterResult<()>;
}

impl<'a> Fun<'a> {
    fn interp_instr(&self, context: &Context<'a>, instr: &Instr<'a>) -> RtlInterpreterResult<()> {
        match instr {
            Instr::EConst(c, r, l) => {
                context.put(r, *c as Value);
                self.interp_label(context, l)
            }
            Instr::ELoad(address_reg, offset, value_reg, l) => {
                let value = context.load(address_reg, offset)?;
                context.put(value_reg, value);
                self.interp_label(context, l)
            }
            Instr::EStore(address_reg, value_reg, offset, l) => {
                let value = context.get(value_reg);
                context.store(address_reg, offset, &value)?;

                self.interp_label(context, l)
            }
            Instr::EMUnop(op, r, l) => {
                match op {
                    Munop::Maddi(c) => {
                        let val = context.get(r);
                        context.put(r, val + c)
                    }
                    Munop::Msetei(c) => {
                        let val = context.get(r);
                        context.put(r, (val == *c).to_minic_bool())
                    }
                    Munop::Msetnei(c) => {
                        let val = context.get(r);
                        context.put(r, (val != *c).to_minic_bool())
                    }
                }
                self.interp_label(context, l)
            }
            Instr::EMBinop(op, r1, r2, l) => {
                match op {
                    Mbinop::MMov => {
                        let val = context.get(r1);
                        context.put(r2, val)
                    }
                    Mbinop::MAdd => {
                        context.put(r2, context.get(r2) + context.get(r1))
                    }
                    Mbinop::MSub => {
                        context.put(r2, context.get(r2) - context.get(r1))
                    }
                    Mbinop::MMul => {
                        context.put(r2, context.get(r2) * context.get(r1))
                    }
                    Mbinop::MDiv => {
                        context.put(r2, context.get(r2) / context.get(r1))
                    }
                    Mbinop::MSete => {
                        let bool = context.get(r2) == context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                    Mbinop::MSetne => {
                        let bool = context.get(r2) != context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                    Mbinop::Msetl => {
                        let bool = context.get(r2) < context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                    Mbinop::Msetle => {
                        let bool = context.get(r2) <= context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                    Mbinop::Msetg => {
                        let bool = context.get(r2) > context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                    Mbinop::Msetge => {
                        let bool = context.get(r2) >= context.get(r1);
                        context.put(r2, bool.to_minic_bool())
                    }
                }
                self.interp_label(context, l)
            }
            Instr::EMuBranch(op, r, l1, l2) => {
                let l = match op {
                    MuBranch::MJz => if !context.get(r).to_bool() { l1 } else { l2 },
                    MuBranch::MJnz => if context.get(r).to_bool() { l1 } else { l2 },
                    MuBranch::MJlei(c) => if context.get(r) <= *c { l1 } else { l2 },
                    MuBranch::MJgi(c) => if context.get(r) > *c { l1 } else { l2 },
                };
                self.interp_label(context, l)
            }
            Instr::EMbBranch(op, r1, r2, l1, l2) => {
                let bool = match op {
                    MbBranch::MJl => context.get(r1) < context.get(r2),
                    MbBranch::MJle => context.get(r1) <= context.get(r2),
                };
                if bool {
                    self.interp_label(context, l1)
                } else {
                    self.interp_label(context, l2)
                }
            }
            Instr::ECall(return_reg, name, args, l) => {
                let fun = context.funs
                    .get(name)
                    .ok_or(RtlInterpreterError::FunctionDoesNotExist(String::from(name.clone())))?;

                let new_context = Context::new(
                    context.stdout.clone(),
                    context.funs.clone(),
                    Rc::new(RefCell::new(HashMap::new())),
                    context.memory.clone(),
                );

                for (fun_reg, arg_reg) in zip(fun.fun_arguments(), args) {
                    new_context.put(fun_reg, context.get(arg_reg));
                }

                fun.interp_fun(&new_context)?;

                context.put(return_reg, new_context.get(fun.fun_result()));
                self.interp_label(context, l)
            }
            Instr::EGoto(l) => self.interp_label(context, l),
        }
    }

    fn interp_label(&self, context: &Context<'a>, label: &Label) -> RtlInterpreterResult<()> {
        if *label == self.exit {
            Ok(())
        } else {
            let instr = self
                .graph
                .instrs
                .get(label)
                .ok_or(RtlInterpreterError::NoSuchInstruction(label.clone()))?
                .clone();

            // println!("Instr {}", instr);
            self.interp_instr(context, &instr)
        }
    }
}

impl<'a> RtlInterpFun<'a> for &'a Fun<'a> {
    fn fun_result(&self) -> &Register {
        return &self.result;
    }

    fn fun_arguments(&self) -> &Vec<Register> {
        &self.arguments
    }

    fn interp_fun(&self, context: &Context<'a>) -> RtlInterpreterResult<()> {
        let entry = &self.entry;
        self.interp_label(context, entry)
    }
}