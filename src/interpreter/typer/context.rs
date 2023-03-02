use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use derive_new::new;
use derive_getters::Getters;
use crate::interpreter::Stdout;
use crate::interpreter::typer::{interp_block, TyperInterpreterResult, Value};
use crate::typer::structure::{BlockIdent, Fun, Ident};

const DEFAULT_FIELD_VALUE: Value = 0;

#[derive(new, Getters)]
pub struct Context<'a> {
    pub vars: MemoryVar<'a>,
    pub memory: Rc<RefCell<HashMap<Value, RefCell<MemoryStruct<'a>>>>>,
    pub functions: Rc<HashMap<Ident<'a>, Box<dyn TyperInterpreterFun<'a> + 'a>>>,
    pub stdout: Rc<RefCell<Stdout>>,
}

#[derive(Debug)]
pub struct MemoryVar<'a> {
    pub vars: HashMap<BlockIdent<'a>, Value>,
}

impl<'x> MemoryVar<'x> {
    pub fn new<'b>() -> MemoryVar<'b> {
        MemoryVar { vars: HashMap::new() }
    }

    pub fn get<'a: 'x>(&mut self, ident: BlockIdent<'a>) -> Value {
        if !self.vars.contains_key(&ident) {
            self.vars.insert(ident.clone(), DEFAULT_FIELD_VALUE);
        }

        *self.vars.get(&ident).unwrap()
    }

    pub fn set<'a: 'x>(&mut self, ident: BlockIdent<'a>, value: Value) {
        self.vars.insert(ident, value);
    }
}

#[derive(Debug)]
pub struct MemoryStruct<'a> {
    pub fields: HashMap<Ident<'a>, Value>,
}

impl<'x> MemoryStruct<'x> {
    pub fn new<'b>() -> MemoryStruct<'b> {
        MemoryStruct { fields: HashMap::new() }
    }

    pub fn get<'a: 'x>(&mut self, ident: Ident<'a>) -> Value {
        if self.fields.contains_key(ident) {
            *self.fields.get(ident).unwrap()
        } else {
            self.fields.insert(ident, DEFAULT_FIELD_VALUE);
            DEFAULT_FIELD_VALUE
        }
    }

    pub fn set<'a: 'x>(&mut self, ident: Ident<'a>, value: Value) {
        self.fields.insert(ident, value);
    }
}

pub trait TyperInterpreterFun<'a> {
    fn call(&self, context: &mut Context<'a>) -> TyperInterpreterResult<Option<Value>>;
}

impl<'a> TyperInterpreterFun<'a> for &'a Fun<'a> {
    fn call(&self, context: &mut Context<'a>) -> TyperInterpreterResult<Option<Value>> {
        interp_block(context, self.block())
    }
}
