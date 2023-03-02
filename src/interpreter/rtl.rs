use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;
use crate::interpreter::rtl::context::Context;
use crate::interpreter::rtl::default::Putchar;
use crate::interpreter::rtl::error::RtlInterpreterError;
use crate::interpreter::Stdout;
use crate::rtl::structure::{File, Fun, Instr, MbBranch, Mbinop, MuBranch, Munop};
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;

pub type RtlInterpreterResult<T> = Result<T, RtlInterpreterError>;

pub fn interp_rtl<'a>(file: &'a File) -> RtlInterpreterResult<Stdout> {
    let main = file.funs().get("main").expect("No main function");

    let mut funs: HashMap<String, Rc<dyn RtlInterpFun + 'a>> = HashMap::new();

    for (name, fun) in file.funs() {
        funs.insert(name.clone(), Rc::new(fun));
    }

    let putchar = Putchar::new();
    funs.insert(String::from("putchar"), Rc::new(putchar));

    let context = Context::new(Stdout::new(),
                               Rc::new(funs),
                               Rc::new(main),
                               RefCell::new(HashMap::new()),
    );

    main.call(&context)?;

    Ok(context.stdout().clone())
}

fn interp_at_label(context: &Context, label: &Label) -> RtlInterpreterResult<()> {
    interp_instr(
        context,
        &context.fun().instr_at_label(label).expect("Called instruction at label which doesn't exist"),
    )
}

fn interp_instr(context: &Context, instr: &Instr) -> RtlInterpreterResult<()> {
    match instr {
        Instr::EConst(c, r, l) => {
            context.put(r, *c as Value);
            interp_at_label(context, l)
        }
        Instr::ELoad(_, _, _, _) => todo!(),
        Instr::EStore(_, _, _, _) => todo!(),
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
            interp_at_label(context, l)
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
            interp_at_label(context, l)
        }
        Instr::EMuBranch(op, r1, l1, l2) => {
            let bool = match op {
                MuBranch::MJz => context.get(r1) == 0,
                MuBranch::MJnz => context.get(r1) != 0,
                MuBranch::MJlei(c) => context.get(r1) <= *c,
                MuBranch::MJgi(c) => context.get(r1) > *c,
            };
            if bool {
                interp_at_label(context, l1)
            } else {
                interp_at_label(context, l2)
            }
        }
        Instr::EMbBranch(op, r1, r2, l1, l2) => {
            let bool = match op {
                MbBranch::MJl => context.get(r1) < context.get(r2),
                MbBranch::MJle => context.get(r1) <= context.get(r2),
            };
            if bool {
                interp_at_label(context, l1)
            } else {
                interp_at_label(context, l2)
            }
        }
        Instr::ECall(r, name, args, l) => {
            let fun = context.funs()
                .get(name)
                .expect("Function doesn't exist");

            for (reg, fun_reg) in zip(fun.fun_arguments(), args) {
                context.put(&reg, context.get(fun_reg));
            }

            let new_context = Context::new(
                context.stdout().clone(),
                context.funs().clone(),
                fun.clone(),
                context.regs().clone(),
            );

            fun.call(&new_context)?;

            context.put(r, context.get(fun.fun_result()));
            interp_at_label(context, l)
        }
        Instr::EGoto(l) => interp_at_label(context, l),
    }
}

type Value = i64;

pub trait RtlInterpFun {
    fn instr_at_label(&self, label: &Label) -> Option<Instr>;
    fn fun_result(&self) -> &Register;
    fn fun_arguments(&self) -> &Vec<Register>;
    fn call(&self, context: &Context) -> RtlInterpreterResult<()>;
}

impl<'a> RtlInterpFun for &'a Fun {
    fn instr_at_label(&self, label: &Label) -> Option<Instr> {
        if *label == *self.exit() {
            None
        } else {
            Some(self.graph().instrs().borrow().get(label).expect("No instruction in graph at label").clone())
        }
    }

    fn fun_result(&self) -> &Register {
        return self.result();
    }

    fn fun_arguments(&self) -> &Vec<Register> {
        self.arguments()
    }

    fn call(&self, context: &Context) -> RtlInterpreterResult<()> {
        let entry = self.entry();
        interp_at_label(context, entry)
    }
}


mod default {
    use crate::interpreter::rtl::context::Context;
    use crate::interpreter::rtl::{RtlInterpFun, RtlInterpreterResult};
    use crate::rtl::structure::{Fresh, Instr};
    use crate::rtl::structure::label::Label;
    use crate::rtl::structure::register::Register;

    pub struct Putchar {
        result: Register,
        args: Vec<Register>,
    }

    impl Putchar {
        pub fn new() -> Putchar {
            Putchar { result: Register::fresh(), args: vec![Register::fresh()] }
        }
    }

    impl RtlInterpFun for Putchar {
        fn instr_at_label(&self, _label: &Label) -> Option<Instr> {
            None
        }

        fn fun_result(&self) -> &Register {
            &self.result
        }

        fn fun_arguments(&self) -> &Vec<Register> {
            &self.args
        }

        fn call(&self, context: &Context) -> RtlInterpreterResult<()> {
            let val = context.get(self.args.first().expect("No first arg to putchar"));
            context.stdout().putchar(val as u8);
            Ok(())
        }
    }
}

mod context {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use derive_new::new;
    use derive_getters::Getters;
    use crate::interpreter::rtl::{RtlInterpFun, Value};
    use crate::interpreter::Stdout;
    use crate::rtl::structure::register::Register;

    const DEFAULT_REGISTER_VALUE: Value = 0;

    #[derive(new, Getters)]
    pub struct Context<'a> {
        stdout: Stdout,
        funs: Rc<HashMap<String, Rc<dyn RtlInterpFun + 'a>>>,
        fun: Rc<dyn RtlInterpFun + 'a>,
        regs: RefCell<HashMap<Register, Value>>,
    }

    impl Context<'_> {
        pub fn put(&self, register: &Register, value: Value) {
            self.regs.borrow_mut().insert(register.clone(), value);
        }

        pub fn get(&self, register: &Register) -> Value {
            *self.regs.borrow_mut().get(register).unwrap_or(&DEFAULT_REGISTER_VALUE)
        }
    }
}

mod error {
    #[derive(Debug)]
    pub struct RtlInterpreterError();
}