use std::collections::HashMap;
use crate::common::{Ident, Stdout};
use crate::typer::interpreter::{interp_block, TyperInterpreterResult, Value};
use crate::typer::structure::{BlockIdent, Fun};

const DEFAULT_FIELD_VALUE: Value = 0;
pub type InterpreterFunctions<'a> = HashMap<Ident<'a>, Box<dyn TyperInterpreterFun<'a> + 'a>>;

pub type InterpreterMemory<'a> = HashMap<Value, MemoryStruct<'a>>;

pub enum InterpreterContext<'a> {
    Root,
    Node(HashMap<BlockIdent<'a>, Value>, Box<InterpreterContext<'a>>)
}

#[derive(Debug)]
pub struct MemoryVar<'a> {
    pub vars: HashMap<BlockIdent<'a>, Value>,
}

#[derive(Debug)]
pub struct MemoryStruct<'a> {
    pub fields: HashMap<Ident<'a>, Value>,
}

impl<'x> InterpreterContext<'x> {

    pub fn new() -> InterpreterContext<'x>{
        return InterpreterContext::Node(HashMap::new(), Box::new(InterpreterContext::Root))
    }

    pub fn get(&self, ident: BlockIdent<'x>) -> Value {
        match self {
            InterpreterContext::Root => {
                DEFAULT_FIELD_VALUE
            },
            InterpreterContext::Node(scope, parent) => {
                scope.get(&ident)
                .map_or_else(|| parent.get(ident),|value| *value)
            }
        }
    }

    pub fn set(&mut self, ident: BlockIdent<'x>, value: Value) {
        match self {
            InterpreterContext::Root => {},
            InterpreterContext::Node(scope, parent) => {
                let _ = scope.insert(ident, value);
            }
        }
    }
}

impl<'x> MemoryStruct<'x> {
    pub fn new<'a>() -> MemoryStruct<'a> {
        MemoryStruct { fields: HashMap::new() }
    }

    pub fn get<'a: 'x>(&self, ident: Ident<'a>) -> Value {
        self.fields.get(ident)
        .map_or_else(|| DEFAULT_FIELD_VALUE, |value| *value)
    }

    pub fn set<'a: 'x>(&mut self, ident: Ident<'a>, value: Value) {
        self.fields.insert(ident, value);
    }
}

pub trait TyperInterpreterFun<'a> {
    fn call(&self, context: &mut InterpreterContext<'a>, functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout) -> TyperInterpreterResult<Option<Value>>;
}

impl<'a> TyperInterpreterFun<'a> for &'a Fun<'a> {
    fn call(&self, context: &mut InterpreterContext<'a>, functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout) -> TyperInterpreterResult<Option<Value>> {
        interp_block(context, functions, memory, stdout, self.block())
    }
}
