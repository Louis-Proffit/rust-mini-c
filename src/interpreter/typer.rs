use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;
use derive_new::new;
use derive_getters::Getters;
use crate::interpreter::Stdout;
use crate::interpreter::typer::defaults::{Malloc, Putchar};
use crate::interpreter::typer::error::TypInterpreterError;
use crate::typer::structure::{Binop, Block, BlockIdent, Expr, ExprNode, File, Fun, Ident, Stmt, Unop};


pub type InterpreterResult = Result<Stdout, TypInterpreterError>;

const DEFAULT_RETURN_VALUE: Value = 0;

#[derive(new, Getters)]
struct Context<'a> {
    vars: MemoryVar<'a>,
    memory: Rc<RefCell<HashMap<Value, RefCell<MemoryStruct<'a>>>>>,
    functions: Rc<HashMap<Ident<'a>, Box<dyn InterpreterCallable<'a> + 'a>>>,
    stdout: Rc<RefCell<Stdout>>,
}

pub fn interp_file<'a>(file: File<'a>) -> InterpreterResult {
    let vars = MemoryVar::new();
    let memory = Rc::new(RefCell::new(HashMap::new()));
    let stdout = Rc::new(RefCell::new(Stdout::new()));

    let mut functions: HashMap<Ident, Box<dyn InterpreterCallable + 'a>> = HashMap::new();
    for (name, fun) in file.into_funs() {
        functions.insert(name.clone(), Box::new(fun));
    }

    functions.insert("putchar", Box::new(Putchar::new()));
    functions.insert("malloc", Box::new(Malloc::new()));

    let functions = Rc::new(functions);

    let mut context = Context::new(
        vars,
        memory,
        functions.clone(),
        stdout.clone(),
    );

    functions.get("main").expect("No main function").call(&mut context);

    let output = stdout.borrow().clone();
    Ok(output)
}

fn interp_stmt<'a>(context: &mut Context<'a>, stmt: &Stmt<'a>) -> Option<Value> {
    match stmt {
        Stmt::SSkip => None,
        Stmt::SExpr(e) => {
            interp_expr(context, e);

            None
        }
        Stmt::SIf(expr, stmt_if, stmt_else) => {
            if let Some(x) = if interp_expr(context, expr).bool() {
                interp_stmt(context, stmt_if)
            } else {
                interp_stmt(context, stmt_else)
            } {
                return Some(x);
            }

            None
        }
        Stmt::SWhile(expr, stmt) => {
            while interp_expr(context, expr).bool() {
                if let Some(x) = interp_stmt(context, stmt) {
                    return Some(x);
                }
            }
            None
        }
        Stmt::SBlock(block) => {
            interp_block(context, block)
        }
        Stmt::SReturn(expr) => {
            Some(interp_expr(context, expr))
        }
    }
}

fn interp_block<'a>(context: &mut Context<'a>, block: &Block<'a>) -> Option<Value> {
    for stmt in block.stmts() {
        if let Some(x) = interp_stmt(context, stmt) {
            return Some(x);
        }
    }

    None
}

fn interp_expr<'a>(context: &mut Context<'a>, expr: &Expr<'a>) -> Value {
    match expr.node() {
        ExprNode::EConst(x) => *x as Value,
        ExprNode::EAccessLocal(x) => {
            context.vars.get(x.clone())
        }
        ExprNode::EAccessField(expr, y) => {
            let address = interp_expr(context, expr);
            context.memory
                .borrow()
                .get(&address)
                .expect("Address doesn't point to anything")
                .borrow_mut()
                .get(y.name().clone())
        }
        ExprNode::EAssignLocal(var, expr) => {
            let value = interp_expr(context, expr);
            context.vars.set(var.clone(), value);
            value
        }
        ExprNode::EAssignField(expr, field, value) => {
            let value = interp_expr(context, value);

            let address = interp_expr(context, expr);

            context.memory
                .borrow_mut()
                .get(&address)
                .expect(&*format!("Empty address {}", address))
                .borrow_mut()
                .set(field.name().clone(), value);

            value
        }
        ExprNode::EUnop(unop, expr) => match unop {
            Unop::UNot => {
                if interp_expr(context, expr).bool() { 0 } else { 1 }
            }
            Unop::UMinus => -interp_expr(context, expr)
        }
        ExprNode::EBinop(binop, expr_1, expr_2) => match binop {
            Binop::BEq => (interp_expr(context, expr_1) == interp_expr(context, expr_2)).to_c_bool(),
            Binop::BNeq => (interp_expr(context, expr_1) != interp_expr(context, expr_2)).to_c_bool(),
            Binop::BLt => (interp_expr(context, expr_1) < interp_expr(context, expr_2)).to_c_bool(),
            Binop::BGt => (interp_expr(context, expr_1) > interp_expr(context, expr_2)).to_c_bool(),
            Binop::BGe => (interp_expr(context, expr_1) >= interp_expr(context, expr_2)).to_c_bool(),
            Binop::BLe => (interp_expr(context, expr_1) <= interp_expr(context, expr_2)).to_c_bool(),
            Binop::BAnd => (interp_expr(context, expr_1).bool() && interp_expr(context, expr_2).bool()).to_c_bool(),
            Binop::BOr => (interp_expr(context, expr_1).bool() || interp_expr(context, expr_2).bool()).to_c_bool(),
            Binop::BAdd => interp_expr(context, expr_1) + interp_expr(context, expr_2),
            Binop::BSub => interp_expr(context, expr_1) - interp_expr(context, expr_2),
            Binop::BMul => interp_expr(context, expr_1) * interp_expr(context, expr_2),
            Binop::BDiv => interp_expr(context, expr_1) / interp_expr(context, expr_2),
        }
        ExprNode::ECall(fun, args) => {
            let mut vars = MemoryVar::new();

            for (formal, arg) in zip(fun.args(), args) {
                vars.set((formal.name(), 0), interp_expr(context, arg));
            }

            let mut new_context = Context::new(
                vars,
                context.memory.clone(),
                context.functions.clone(),
                context.stdout.clone(),
            );

            context.functions
                .get(fun.profile().name())
                .expect("Function doesn't exist")
                .call(&mut new_context)
                .unwrap_or(DEFAULT_RETURN_VALUE)
        }
        ExprNode::ESizeof(x) => x.c_size() as Value
    }
}

type Value = i64;

const DEFAULT_VALUE: i64 = 0;

#[derive(Debug)]
struct MemoryVar<'a> {
    vars: HashMap<BlockIdent<'a>, Value>,
}

impl<'x> MemoryVar<'x> {
    fn new<'b>() -> MemoryVar<'b> {
        MemoryVar { vars: HashMap::new() }
    }

    fn get<'a: 'x>(&mut self, ident: BlockIdent<'a>) -> Value {
        if !self.vars.contains_key(&ident) {
            self.vars.insert(ident, DEFAULT_VALUE);
        }

        *self.vars.get(&ident).unwrap()
    }

    fn set<'a: 'x>(&mut self, ident: BlockIdent<'a>, value: Value) {
        self.vars.insert(ident, value);
    }
}

#[derive(Debug)]
struct MemoryStruct<'a> {
    fields: HashMap<Ident<'a>, Value>,
}

impl<'x> MemoryStruct<'x> {
    fn new<'b>() -> MemoryStruct<'b> {
        MemoryStruct { fields: HashMap::new() }
    }

    fn get<'a: 'x>(&mut self, ident: Ident<'a>) -> Value {
        if self.fields.contains_key(ident) {
            *self.fields.get(ident).unwrap()
        } else {
            self.fields.insert(ident, DEFAULT_VALUE);
            DEFAULT_VALUE
        }
    }

    fn set<'a: 'x>(&mut self, ident: Ident<'a>, value: Value) {
        self.fields.insert(ident, value);
    }
}

trait Bool {
    fn bool(&self) -> bool;
}

impl Bool for Value {
    fn bool(&self) -> bool {
        *self != 0
    }
}

trait ToCBool {
    fn to_c_bool(&self) -> Value;
}

impl ToCBool for bool {
    fn to_c_bool(&self) -> Value {
        if *self { 1 } else { 0 }
    }
}

impl<'a> InterpreterCallable<'a> for Fun<'a> {
    fn call(&self, context: &mut Context<'a>) -> Option<Value> {
        interp_block(context, self.block())
    }
}

trait InterpreterCallable<'a> {
    fn call(&self, context: &mut Context<'a>) -> Option<Value>;
}

pub mod defaults {
    use std::cell::RefCell;
    use std::sync::Mutex;
    use derive_new::new;
    use crate::interpreter::typer::{Context, InterpreterCallable, MemoryStruct, Value};

    static MEMORY_INDEX: Mutex<i64> = Mutex::new(1);

    #[derive(new)]
    pub struct Malloc {}

    #[derive(new)]
    pub struct Putchar {}

    impl<'a> InterpreterCallable<'a> for Putchar {
        fn call(&self, context: &mut Context<'a>) -> Option<Value> {
            let value = context.vars.get(("c", 0)); // TODO arg;
            context.stdout.borrow_mut().putchar(value as u8);

            Some(value)
        }
    }

    impl<'a> InterpreterCallable<'a> for Malloc {
        fn call(&self, context: &mut Context<'a>) -> Option<Value> {
            let mut index = MEMORY_INDEX.lock().unwrap();

            context.memory.borrow_mut().insert(*index, RefCell::new(MemoryStruct::new()));

            let old_index = index.clone();
            *index += 1;

            Some(old_index)
        }
    }
}

pub mod error {
    #[derive(Debug)]
    pub struct TypInterpreterError {}
}