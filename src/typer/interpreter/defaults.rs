use std::sync::Mutex;
use crate::common::Stdout;
use crate::typer::interpreter::context::{InterpreterContext, MemoryStruct, TyperInterpreterFun};
use crate::typer::interpreter::{TyperInterpreterResult, Value};
use crate::typer::structure::BlockIdent;

use super::context::{InterpreterFunctions, InterpreterMemory};

static MALLOC_MEMORY_INDEX: Mutex<Value> = Mutex::new(1);

pub struct Malloc();

pub struct Putchar();

impl<'a> TyperInterpreterFun<'a> for Putchar {
    fn call(&self, context: &mut InterpreterContext<'a>, _functions: &InterpreterFunctions<'a>, _memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout) -> TyperInterpreterResult<Option<Value>> {
        let value = context.get(BlockIdent::Arg(0, "c"));
        stdout.putchar(value as u8 as char);
        Ok(Some(value))
    }
}

impl<'a> TyperInterpreterFun<'a> for Malloc {
    fn call(&self, _context: &mut InterpreterContext<'a>, _functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, _stdout:&mut Stdout) -> TyperInterpreterResult<Option<Value>> {
        let mut index = MALLOC_MEMORY_INDEX.lock().unwrap();

        memory.insert(*index, MemoryStruct::new());

        let old_index = index.clone();
        *index += 1;

        Ok(Some(old_index))
    }
}
