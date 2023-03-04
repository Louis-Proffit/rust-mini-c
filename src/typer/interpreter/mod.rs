pub mod defaults;
pub mod error;
mod context;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::common::bool::{Bool, ToCBool};
use crate::common::{Ident, MALLOC, PUTCHAR, Stdout};
use crate::typer::interpreter::context::{Context, MemoryVar, TyperInterpreterFun};
use crate::typer::interpreter::defaults::{Malloc, Putchar};
use crate::typer::interpreter::error::TypInterpreterError;
use crate::typer::structure::{Binop, Block, Expr, ExprNode, File, Stmt, Unop};

const DEFAULT_RETURN_VALUE: Value = 0;
type Value = i64;

pub type TyperInterpreterResult<T> = Result<T, TypInterpreterError>;

pub fn interp_typed_file<'a>(file: &'a File<'a>) -> TyperInterpreterResult<Stdout> {
    let vars = MemoryVar::new();
    let memory = Rc::new(RefCell::new(HashMap::new()));
    let stdout = Rc::new(RefCell::new(Stdout::new()));

    let mut functions: HashMap<Ident, Box<dyn TyperInterpreterFun + 'a>> = HashMap::new();
    for (name, fun) in file.funs() {
        functions.insert(name.clone(), Box::new(fun));
    }

    functions.insert(PUTCHAR, Box::new(Putchar()));
    functions.insert(MALLOC, Box::new(Malloc()));

    let functions = Rc::new(functions);

    let mut context = Context::new(
        vars,
        memory,
        functions.clone(),
        stdout.clone(),
    );

    functions.get("main").expect("No main function").call(&mut context)?;

    let output = stdout.borrow().clone();
    Ok(output)
}

fn interp_stmt<'a>(context: &mut Context<'a>, stmt: &Stmt<'a>) -> TyperInterpreterResult<Option<Value>> {
    match stmt {
        Stmt::SSkip => Ok(None),
        Stmt::SExpr(e) => {
            let _ = interp_expr(context, e)?;

            Ok(None)
        }
        Stmt::SIf(expr, stmt_if, stmt_else) => {
            if let Some(x) = if interp_expr(context, expr)?.to_bool() {
                interp_stmt(context, stmt_if)?
            } else {
                interp_stmt(context, stmt_else)?
            } {
                return Ok(Some(x));
            }

            Ok(None)
        }
        Stmt::SWhile(expr, stmt) => {
            while interp_expr(context, expr)?.to_bool() {
                if let Some(x) = interp_stmt(context, stmt)? {
                    return Ok(Some(x));
                }
            }
            Ok(None)
        }
        Stmt::SBlock(block) => {
            interp_block(context, block)
        }
        Stmt::SReturn(expr) => {
            Ok(Some(interp_expr(context, expr)?))
        }
    }
}

fn interp_block<'a>(context: &mut Context<'a>, block: &Block<'a>) -> TyperInterpreterResult<Option<Value>> {
    for stmt in block.stmts() {
        if let Some(x) = interp_stmt(context, stmt)? {
            return Ok(Some(x));
        }
    }

    Ok(None)
}

fn interp_expr<'a>(context: &mut Context<'a>, expr: &Expr<'a>) -> TyperInterpreterResult<Value> {
    match expr.node() {
        ExprNode::EConst(x) => Ok(*x as Value),
        ExprNode::EAccessLocal(x) => {
            Ok(context.vars.get(x.clone()))
        }
        ExprNode::EAccessField(expr, y) => {
            let address = interp_expr(context, expr)?;
            Ok(context.memory
                .borrow()
                .get(&address)
                .expect("Address doesn't point to anything")
                .borrow_mut()
                .get(y.name().clone()))
        }
        ExprNode::EAssignLocal(var, expr) => {
            let value = interp_expr(context, expr)?;
            context.vars.set(var.clone(), value);
            Ok(value)
        }
        ExprNode::EAssignField(expr, field, value) => {
            let value = interp_expr(context, value)?;

            let address = interp_expr(context, expr)?;

            context.memory
                .borrow_mut()
                .get(&address)
                .expect(&*format!("Empty address {}", address))
                .borrow_mut()
                .set(field.name().clone(), value);

            Ok(value)
        }
        ExprNode::EUnop(unop, expr) => Ok(match unop {
            Unop::UNot => {
                if interp_expr(context, expr)?.to_bool() { 0 } else { 1 }
            }
            Unop::UMinus => -interp_expr(context, expr)?
        }),
        ExprNode::EBinop(binop, expr_1, expr_2) => Ok(
            match binop {
                Binop::BEq => (interp_expr(context, expr_1)? == interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BNeq => (interp_expr(context, expr_1)? != interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BLt => (interp_expr(context, expr_1)? < interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BGt => (interp_expr(context, expr_1)? > interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BGe => (interp_expr(context, expr_1)? >= interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BLe => (interp_expr(context, expr_1)? <= interp_expr(context, expr_2)?).to_minic_bool(),
                Binop::BAnd => (interp_expr(context, expr_1)?.to_bool() && interp_expr(context, expr_2)?.to_bool()).to_minic_bool(),
                Binop::BOr => (interp_expr(context, expr_1)?.to_bool() || interp_expr(context, expr_2)?.to_bool()).to_minic_bool(),
                Binop::BAdd => interp_expr(context, expr_1)? + interp_expr(context, expr_2)?,
                Binop::BSub => interp_expr(context, expr_1)? - interp_expr(context, expr_2)?,
                Binop::BMul => interp_expr(context, expr_1)? * interp_expr(context, expr_2)?,
                Binop::BDiv => interp_expr(context, expr_1)? / interp_expr(context, expr_2)?,
            }
        ),
        ExprNode::ECall(fun, args) => {
            let mut vars = MemoryVar::new();

            for arg in args {
                vars.set(arg.formal().name().clone(), interp_expr(context, arg.expr())?);
            }

            let mut new_context = Context::new(
                vars,
                context.memory.clone(),
                context.functions.clone(),
                context.stdout.clone(),
            );

            Ok(context.functions
                .get(fun.name())
                .expect("Function doesn't exist")
                .call(&mut new_context)?
                .unwrap_or(DEFAULT_RETURN_VALUE))
        }
    }
}
