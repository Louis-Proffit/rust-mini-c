use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use derive_new::new;
use crate::common::{Ident, StackOffset, Stdout, Value};
use crate::rtl::interpreter::{RtlInterpFun, RtlInterpreterResult};
use crate::rtl::interpreter::error::RtlInterpreterError;
use crate::rtl::structure::register::PseudoRegister;

const DEFAULT_FIELD_VALUE: Value = 0;
const DEFAULT_REGISTER_VALUE: Value = 0;

#[derive(new)]
pub struct Context<'a> {
    pub stdout: Rc<Stdout>,
    pub funs: Rc<HashMap<Ident<'a>, Rc<dyn RtlInterpFun<'a> + 'a>>>,
    pub regs: Rc<RefCell<HashMap<PseudoRegister, Value>>>,
    pub memory: Rc<RefCell<HashMap<Value, HashMap<StackOffset, Value>>>>,
}

impl Context<'_> {
    pub fn put(&self, register: &PseudoRegister, value: Value) {
        self.regs.borrow_mut().insert(register.clone(), value);
    }

    pub fn get(&self, register: &PseudoRegister) -> Value {
        *self.regs.borrow_mut().get(register).unwrap_or(&DEFAULT_REGISTER_VALUE)
    }

    pub fn load(&self, register: &PseudoRegister, offset: &StackOffset) -> RtlInterpreterResult<Value> {
        let address = self.get(register);

        Ok(*self.memory
            .borrow()
            .get(&address)
            .ok_or(RtlInterpreterError::UnallocatedMemory(address))?
            .get(offset)
            .unwrap_or(&DEFAULT_FIELD_VALUE))
    }

    pub fn store(&self, register: &PseudoRegister, offset: &StackOffset, value: &Value) -> RtlInterpreterResult<()> {
        let address = self.get(register);

        self.memory
            .borrow_mut()
            .get_mut(&address)
            .ok_or(RtlInterpreterError::UnallocatedMemory(address))?
            .insert(*offset, *value);

        Ok(())
    }
}
