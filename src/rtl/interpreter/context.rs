use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use derive_new::new;
use derive_getters::Getters;
use crate::common::{Stdout, Value};
use crate::rtl::interpreter::RtlInterpFun;
use crate::rtl::structure::register::Register;

const DEFAULT_REGISTER_VALUE: Value = 0;

#[derive(new, Getters)]
pub struct Context<'a> {
    stdout: Rc<Stdout>,
    funs: Rc<HashMap<String, Rc<dyn RtlInterpFun + 'a>>>,
    fun: Rc<dyn RtlInterpFun + 'a>,
    regs: Rc<RefCell<HashMap<Register, Value>>>,
    memory: Rc<RefCell<HashMap<Value, HashMap<usize, Value>>>>,
}

impl Context<'_> {
    pub fn put(&self, register: &Register, value: Value) {
        self.regs.borrow_mut().insert(register.clone(), value);
    }

    pub fn get(&self, register: &Register) -> Value {
        *self.regs.borrow_mut().get(register).unwrap_or(&DEFAULT_REGISTER_VALUE)
    }
}
