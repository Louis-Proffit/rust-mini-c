mod default;
mod context;
mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;
use crate::interpreter::rtl::context::Context;
use crate::interpreter::rtl::error::RtlInterpreterError;
use crate::interpreter::{MAIN, MALLOC, PUTCHAR, Stdout};
use crate::interpreter::rtl::default::malloc::Malloc;
use crate::interpreter::rtl::default::putchar::Putchar;
use crate::rtl::structure::{File, Fun, Instr, MbBranch, Mbinop, MuBranch, Munop};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

pub type RtlInterpreterResult<T> = Result<T, RtlInterpreterError>;

pub fn interp_rtl<'a>(file: &'a File) -> RtlInterpreterResult<Stdout> {
    let main = file.funs().get(MAIN).ok_or(RtlInterpreterError::FunctionDoesNotExist(String::from(MAIN)))?;

    let mut funs: HashMap<String, Rc<dyn RtlInterpFun + 'a>> = HashMap::new();

    for (name, fun) in file.funs() {
        funs.insert(name.clone(), Rc::new(fun));
    }

    funs.insert(String::from(PUTCHAR), Rc::new(Putchar::new()));
    funs.insert(String::from(MALLOC), Rc::new(Malloc::new()));

    let context = Context::new(
        Rc::new(Stdout::new()),
        Rc::new(funs),
        Rc::new(main),
        RefCell::new(HashMap::new()),
        RefCell::new(HashMap::new()),
    );

    main.interp_fun(&context)?;

    Ok(context.stdout().as_ref().clone())
}

fn interp_instr(context: &Context, instr: &Instr) -> RtlInterpreterResult<()> {
    match instr {
        Instr::EConst(c, r, l) => {
            context.put(r, *c as Value);
            context.fun().interp_label(context, l)
        }
        Instr::ELoad(address_reg, offset, value_reg, l) => {
            let address = context.get(address_reg);

            let value = context.memory()
                .borrow_mut()
                .get(&address)
                .unwrap()
                .get(offset)
                .unwrap_or(&0)
                .clone();

            context.put(value_reg, value);

            context.fun().interp_label(context, l)
        }
        Instr::EStore(address_reg, value_reg, offset, l) => {
            let address = context.get(address_reg);
            let value = context.get(value_reg);

            context.memory()
                .borrow_mut()
                .get_mut(&address)
                .unwrap()
                .insert(*offset, value);

            context.fun().interp_label(context, l)
        }
        Instr::EMUnop(op, r, l) => {
            match op {
                Munop::Maddi(x) => {
                    let val = context.get(r);
                    context.put(r, val + x)
                }
                Munop::Msetei(x) => {
                    let val = context.get(r);
                    context.put(r, if val == *x { 1 } else { 0 })
                }
                Munop::Msetnei(x) => {
                    let val = context.get(r);
                    context.put(r, if val == *x { 0 } else { 1 })
                }
            }
            context.fun().interp_label(context, l)
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
                    let bool = context.get(r1) == context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
                Mbinop::MSetne => {
                    let bool = context.get(r1) != context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
                Mbinop::Msetl => {
                    let bool = context.get(r1) < context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
                Mbinop::Msetle => {
                    let bool = context.get(r1) <= context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
                Mbinop::Msetg => {
                    let bool = context.get(r1) > context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
                Mbinop::Msetge => {
                    let bool = context.get(r1) >= context.get(r2);
                    context.put(r2, if bool { 1 } else { 0 })
                }
            }
            context.fun().interp_label(context, l)
        }
        Instr::EMuBranch(op, r1, l1, l2) => {
            let bool = match op {
                MuBranch::MJz => context.get(r1) == 0,
                MuBranch::MJnz => context.get(r1) != 0,
                MuBranch::MJlei(c) => context.get(r1) <= *c,
                MuBranch::MJgi(c) => context.get(r1) > *c,
            };
            if bool {
                context.fun().interp_label(context, l1)
            } else {
                context.fun().interp_label(context, l2)
            }
        }
        Instr::EMbBranch(op, r1, r2, l1, l2) => {
            let bool = match op {
                MbBranch::MJl => context.get(r1) < context.get(r2),
                MbBranch::MJle => context.get(r1) <= context.get(r2),
            };
            if bool {
                context.fun().interp_label(context, l1)
            } else {
                context.fun().interp_label(context, l2)
            }
        }
        Instr::ECall(r, name, args, l) => {
            let fun = context.funs()
                .get(name)
                .ok_or(RtlInterpreterError::FunctionDoesNotExist(name.clone()))?;

            for (reg, fun_reg) in zip(fun.fun_arguments(), args) {
                context.put(&reg, context.get(fun_reg));
            }

            let new_context = Context::new(
                context.stdout().clone(),
                context.funs().clone(),
                fun.clone(),
                context.regs().clone(),
                context.memory().clone(),
            );

            fun.interp_fun(&new_context)?;

            context.put(r, context.get(fun.fun_result()));
            context.fun().interp_label(context, l)
        }
        Instr::EGoto(l) => context.fun().interp_label(context, l),
    }
}

type Value = i64;

pub trait RtlInterpFun {
    fn interp_label(&self, context: &Context, label: &Label) -> RtlInterpreterResult<()>;
    fn fun_result(&self) -> &Register;
    fn fun_arguments(&self) -> &Vec<Register>;
    fn interp_fun(&self, context: &Context) -> RtlInterpreterResult<()>;
}

impl<'a> RtlInterpFun for &'a Fun {
    fn interp_label(&self, context: &Context, label: &Label) -> RtlInterpreterResult<()> {
        if *label == *self.exit() {
            Ok(())
        } else {
            let instr = self
                .graph()
                .instrs()
                .borrow()
                .get(label)
                .ok_or(RtlInterpreterError::NoSuchInstruction(label.clone()))?
                .clone();

            interp_instr(context, &instr)
        }
    }

    fn fun_result(&self) -> &Register {
        return self.result();
    }

    fn fun_arguments(&self) -> &Vec<Register> {
        self.arguments()
    }

    fn interp_fun(&self, context: &Context) -> RtlInterpreterResult<()> {
        let entry = self.entry();
        self.interp_label(context, entry)
    }
}