use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use derive_getters::Getters;
use derive_new::new;
use crate::common::Ident;
use crate::typer::structure::{BlockIdent, Formal, Signature, Struct, Typ};

#[derive(new, Debug, Getters, Clone)]
pub struct FileContext<'a> {
    structs: Rc<RefCell<HashMap<Ident<'a>, Rc<Struct<'a>>>>>,
    funs: Rc<RefCell<HashMap<Ident<'a>, Rc<Signature<'a>>>>>,
}

impl FileContext<'_> {
    pub fn default<'a>() -> FileContext<'a> {
        let mut funs = HashMap::new();

        let putchar = Signature::putchar();
        let malloc = Signature::malloc();

        funs.insert(
            putchar.name().clone(),
            Rc::new(putchar),
        );

        funs.insert(
            malloc.name().clone(),
            Rc::new(malloc),
        );

        let structs = Rc::new(RefCell::new(HashMap::new()));
        let funs = Rc::new(RefCell::new(funs));

        FileContext::new(structs, funs)
    }
}

pub trait ParentContext<'a>: Debug {
    fn declare(&self, ident: BlockIdent<'a>);
    fn typ(&self, ident: Ident<'a>) -> Option<Typ<'a>>;
    fn fresh_index(&self) -> u8;
    fn context(&self) -> Rc<FileContext<'a>>;
    fn fun_typ(&self) -> Typ<'a>;
    fn fun_name(&self) -> Ident<'a>;
    fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a>;
}

#[derive(Debug, Clone, Getters)]
pub struct FunctionContext<'a> {
    context: Rc<FileContext<'a>>,
    name: Ident<'a>,
    typ: Typ<'a>,
    block_counter: RefCell<u8>,
    arguments: RefCell<Vec<Formal<'a>>>,
    locals: RefCell<HashSet<BlockIdent<'a>>>,
}

impl<'x> FunctionContext<'x> {
    const ARGUMENT_BLOCK_INDEX: u8 = 0;

    pub fn new<'a>(
        context: Rc<FileContext<'a>>,
        name: Ident<'a>,
        typ: Typ<'a>,
        arguments: Vec<Formal<'a>>,
    ) -> FunctionContext<'a> {
        let block_counter = RefCell::new(FunctionContext::ARGUMENT_BLOCK_INDEX + 1);
        let arguments = RefCell::new(arguments);
        let locals = RefCell::new(HashSet::new());

        FunctionContext {
            context,
            name,
            typ,
            block_counter,
            arguments,
            locals,
        }
    }

    fn find_one_by_ident(&self, ident: Ident<'x>) -> Option<Formal<'x>> {
        self.arguments.borrow().iter().find(|name| {
            match name.name() {
                BlockIdent::Arg(_, name) if *name == ident => true,
                _ => false
            }
        }).cloned()
    }
}

impl<'a> ParentContext<'a> for FunctionContext<'a> {
    fn declare(&self, ident: BlockIdent<'a>) {
        self.locals.borrow_mut().insert(ident);
    }

    fn typ(&self, ident: Ident<'a>) -> Option<Typ<'a>> {
        self.find_one_by_ident(ident).map(|x| x.typ().clone())
    }

    fn fresh_index(&self) -> u8 {
        let mut fresh = self.block_counter.borrow_mut();
        let returned = *fresh.deref();
        *fresh += 1;
        returned
    }

    fn context(&self) -> Rc<FileContext<'a>> {
        self.context.clone()
    }

    fn fun_typ(&self) -> Typ<'a> {
        self.typ.clone()
    }

    fn fun_name(&self) -> Ident<'a> {
        self.name.clone()
    }

    fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a> {
        self.find_one_by_ident(ident).expect("ident doesn't exist").name().clone()
    }
}

#[derive(Debug, Clone, Getters)]
pub struct BlockContext<'a> {
    context: Rc<FileContext<'a>>,
    index: u8,
    parent: Rc<dyn ParentContext<'a> + 'a>,
    vars: HashMap<Ident<'a>, Typ<'a>>,
}

impl BlockContext<'_> {
    pub fn new<'a>(context: Rc<FileContext<'a>>,
                   parent: Rc<dyn ParentContext<'a> + 'a>,
                   vars: HashMap<Ident<'a>, Typ<'a>>) -> BlockContext<'a> {
        let index = parent.fresh_index();

        for (name, _) in &vars {
            parent.declare(BlockIdent::Local(index, name.clone()))
        }

        BlockContext {
            context,
            parent,
            vars,
            index,
        }
    }
}

impl<'a> ParentContext<'a> for BlockContext<'a> {
    fn declare(&self, ident: BlockIdent<'a>) {
        self.parent.declare(ident)
    }

    fn typ(&self, ident: Ident<'a>) -> Option<Typ<'a>> {
        match self.vars.get(ident) {
            None => self.parent.typ(ident),
            Some(x) => Some(x.clone())
        }
    }

    fn fresh_index(&self) -> u8 {
        self.parent.fresh_index()
    }

    fn context(&self) -> Rc<FileContext<'a>> {
        self.context.clone()
    }

    fn fun_typ(&self) -> Typ<'a> {
        self.parent.fun_typ()
    }

    fn fun_name(&self) -> Ident<'a> {
        self.parent.fun_name()
    }

    fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a> {
        if self.vars.contains_key(ident) {
            BlockIdent::Local(
                self.index,
                ident,
            )
        } else {
            self.parent.get_block_ident(ident)
        }
    }
}
