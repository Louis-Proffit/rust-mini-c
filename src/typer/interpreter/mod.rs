pub mod defaults;
pub mod error;
mod context;

use std::collections::HashMap;
use context::InterpreterFunctions;
use crate::common::bool::{Bool, ToCBool};
use crate::common::{MALLOC, PUTCHAR, Stdout};
use crate::rtl::structure::BlockIdent;
use crate::typer::interpreter::context::InterpreterContext;
use crate::typer::interpreter::defaults::{Malloc, Putchar};
use crate::typer::interpreter::error::TypInterpreterError;
use crate::typer::structure::{Binop, Block, Expr, ExprNode, File, Stmt, Unop};

use self::context::InterpreterMemory;

const DEFAULT_RETURN_VALUE: Value = 0;
type Value = i64;

pub type TyperInterpreterResult<T> = Result<T, TypInterpreterError>;

pub fn interp_typed_file<'a>(file: &'a File<'a>) -> TyperInterpreterResult<Stdout> {
    let mut functions: InterpreterFunctions = HashMap::new();

    for (name, fun) in file.funs() {
        functions.insert(name, Box::new(fun));
    }

    functions.insert(PUTCHAR, Box::new(Putchar()));
    functions.insert(MALLOC, Box::new(Malloc()));

    let functions = functions;

    let mut context = InterpreterContext::new();

    let mut stdout = Stdout::new();
    let mut memory = HashMap::new();

    let main =  functions.get("main").expect("No main function");
    main.call(&mut context, &functions, &mut memory, &mut stdout)?;

    Ok(stdout)
}

fn interp_stmt<'a>(context: &mut InterpreterContext<'a>, functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout, stmt: &Stmt<'a>) -> TyperInterpreterResult<Option<Value>> {
    match stmt {
        Stmt::SSkip => Ok(None),
        Stmt::SExpr(e) => {
            let _ = interp_expr(context, functions, memory, stdout, e)?;

            Ok(None)
        }
        Stmt::SIf(expr, stmt_if, stmt_else) => {
            if let Some(x) = if interp_expr(context, functions, memory, stdout, expr)?.to_bool() {
                interp_stmt(context, functions, memory, stdout, stmt_if)?
            } else {
                interp_stmt(context, functions, memory, stdout, stmt_else)?
            } {
                return Ok(Some(x));
            }

            Ok(None)
        }
        Stmt::SWhile(expr, stmt) => {
            while interp_expr(context, functions, memory, stdout, expr)?.to_bool() {
                if let Some(x) = interp_stmt(context, functions, memory, stdout, stmt)? {
                    return Ok(Some(x));
                }
            }
            Ok(None)
        }
        Stmt::SBlock(block) => {
            interp_block(context, functions, memory, stdout, block)
        }
        Stmt::SReturn(expr) => {
            Ok(Some(interp_expr(context, functions, memory, stdout, expr)?))
        }
    }
}

fn interp_block<'a>(context: &mut InterpreterContext<'a>, functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout, block: &Block<'a>) -> TyperInterpreterResult<Option<Value>> {
    for stmt in block.stmts() {
        if let Some(x) = interp_stmt(context, functions, memory, stdout, stmt)? {
            return Ok(Some(x));
        }
    }

    Ok(None)
}

fn interp_expr<'a>(context: &mut InterpreterContext<'a>, functions:&InterpreterFunctions<'a>, memory: &mut InterpreterMemory<'a>, stdout:&mut Stdout, expr: &Expr<'a>) -> TyperInterpreterResult<Value> {
    match expr.node() {
        ExprNode::EConst(x) => Ok(*x as Value),
        ExprNode::EAccessLocal(x) => {
            Ok(context.get(x.clone()))
        }
        ExprNode::EAccessField(expr, y) => {
            let address = interp_expr(context, functions, memory, stdout, expr)?;
            Ok(memory
                .get(&address)
                .expect("Address doesn't point to anything")
                .get(y.name()))
        }
        ExprNode::EAssignLocal(var, expr) => {
            let value = interp_expr(context, functions, memory, stdout, expr)?;
            context.set(var.clone(), value);
            Ok(value)
        }
        ExprNode::EAssignField(expr, field, value) => {
            let value = interp_expr(context, functions, memory, stdout, value)?;

            let address = interp_expr(context, functions, memory, stdout, expr)?;

            memory
                .get_mut(&address)
                .expect(&*format!("Empty address {}", address))
                .set(field.name(), value);

            Ok(value)
        }
        ExprNode::EUnop(unop, expr) => Ok(match unop {
            Unop::UNot => {
                if interp_expr(context, functions, memory, stdout, expr)?.to_bool() { 0 } else { 1 }
            }
            Unop::UMinus => -interp_expr(context, functions, memory, stdout, expr)?
        }),
        ExprNode::EBinop(binop, expr_1, expr_2) => Ok(
            match binop {
                Binop::BEq => (interp_expr(context, functions, memory, stdout, expr_1)? == interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BNeq => (interp_expr(context, functions, memory, stdout, expr_1)? != interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BLt => (interp_expr(context, functions, memory, stdout, expr_1)? < interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BGt => (interp_expr(context, functions, memory, stdout, expr_1)? > interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BGe => (interp_expr(context, functions, memory, stdout, expr_1)? >= interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BLe => (interp_expr(context, functions, memory, stdout, expr_1)? <= interp_expr(context, functions, memory, stdout, expr_2)?).to_minic_bool(),
                Binop::BAnd => (interp_expr(context, functions, memory, stdout, expr_1)?.to_bool() && interp_expr(context, functions, memory, stdout, expr_2)?.to_bool()).to_minic_bool(),
                Binop::BOr => (interp_expr(context, functions, memory, stdout, expr_1)?.to_bool() || interp_expr(context, functions, memory, stdout, expr_2)?.to_bool()).to_minic_bool(),
                Binop::BAdd => interp_expr(context, functions, memory, stdout, expr_1)? + interp_expr(context, functions, memory, stdout, expr_2)?,
                Binop::BSub => interp_expr(context, functions, memory, stdout, expr_1)? - interp_expr(context, functions, memory, stdout, expr_2)?,
                Binop::BMul => interp_expr(context, functions, memory, stdout, expr_1)? * interp_expr(context, functions, memory, stdout, expr_2)?,
                Binop::BDiv => interp_expr(context, functions, memory, stdout, expr_1)? / interp_expr(context, functions, memory, stdout, expr_2)?,
            }
        ),
        ExprNode::ECall(fun, args) => {
            let mut new_context:InterpreterContext<'a> = InterpreterContext::new();

            for arg in args {
                let name: BlockIdent<'a> = arg.formal().name().clone();
                let value = interp_expr(context, functions, memory, stdout, arg.expr())?;
                new_context.set(
                    name,
                    value
                )
            }

            Ok(functions
                .get(fun.name())
                .expect("Function doesn't exist")
                .call(&mut new_context, functions, memory, stdout)?
                .unwrap_or(DEFAULT_RETURN_VALUE))
        }
    }
}
