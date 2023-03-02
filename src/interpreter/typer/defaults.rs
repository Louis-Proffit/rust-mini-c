use std::cell::RefCell;
use std::sync::Mutex;
use crate::interpreter::typer::{Context, TyperInterpreterFun, TyperInterpreterResult, Value};
use crate::interpreter::typer::context::MemoryStruct;
use crate::interpreter::typer::error::TypInterpreterError;
use crate::typer::structure::BlockIdent;

static MALLOC_MEMORY_INDEX: Mutex<Value> = Mutex::new(1);

pub struct Malloc();

pub struct Putchar();

impl<'a> TyperInterpreterFun<'a> for Putchar {
    fn call(&self, context: &mut Context<'a>) -> TyperInterpreterResult<Option<Value>> {
        for (ident, value) in &context.vars.vars {
            match ident {
                BlockIdent::Arg(0, "c") => {
                    context.stdout.borrow_mut().putchar(*value as u8);
                    return Ok(Some(*value));
                }
                _ => {}
            }
        }
        Err(TypInterpreterError::new())
    }
}

impl<'a> TyperInterpreterFun<'a> for Malloc {
    fn call(&self, context: &mut Context<'a>) -> TyperInterpreterResult<Option<Value>> {
        let mut index = MALLOC_MEMORY_INDEX.lock().unwrap();

        context.memory.borrow_mut().insert(*index, RefCell::new(MemoryStruct::new()));

        let old_index = index.clone();
        *index += 1;

        Ok(Some(old_index))
    }
}
