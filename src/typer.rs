use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use itertools::{EitherOrBoth, Itertools};
use structure::*;
use crate::parser::structure as parser;
use crate::typer::context::{BlockContext, FileContext, FunctionContext, ParentContext};
use crate::typer::error::{DuplicateArgName, DuplicateFieldName, IncompatibleTyp, TypError};

pub type TypResult<'a, T> = Result<T, TypError<'a>>;

pub fn typ_file<'a>(context: Rc<FileContext<'a>>, file: &'a parser::File<'a>) -> TypResult<'a, File<'a>> {
    for structure in file.structs() {
        let name = structure.name();

        let structure = typ_struct(context.clone(), structure)?;

        if let Some(_) = context.structs().borrow_mut().insert(name, Rc::new(structure)) {
            return Err(TypError::DuplicateStructName(name));
        }
    }

    let mut funs = HashMap::new();

    for fun in file.funs() {
        let name = fun.profile().name();

        let fun = typ_fun(context.clone(), fun)?;
        funs.insert(name.clone(), fun);
    }

    Ok(check_main(File::new(funs))?)
}

fn check_main(file: File) -> TypResult<File> {
    match file.funs().get("main") {
        None => Err(TypError::MissingMainFunction),
        Some(fun) => {
            let signature = fun.signature();
            if signature.profile().typ() != &Typ::TInt {
                Err(TypError::WrongMainFunctionSignature)
            } else if !signature.args().is_empty() {
                Err(TypError::WrongMainFunctionSignature)
            } else {
                Ok(file)
            }
        }
    }
}

fn typ_struct<'a>(context: Rc<FileContext<'a>>, structure: &'a parser::Struct<'a>) -> TypResult<'a, Struct<'a>> {
    let struct_name = structure.name();

    let mut structs = HashMap::new();

    for (k, v) in context.structs().borrow().iter() {
        structs.insert(k.clone(), v.clone());
    }

    let fields = Rc::new(RefCell::new(HashMap::new()));
    let new_struct = Rc::new(Struct::new(struct_name, fields.clone()));
    structs.insert(struct_name.clone(), new_struct.clone());

    let struct_context = Rc::new(FileContext::new(
        Rc::new(structs.into()),
        context.funs().clone(),
    ));

    for field in structure.fields() {
        let field_name = field.name();

        if let Some(_) = fields.borrow_mut().insert(
            field_name.clone(),
            Rc::new(Field::new(
                field_name,
                typ_typ(struct_context.clone(), field.typ())?.clone(),
            )),
        ) {
            return Err(TypError::DuplicateFieldName(
                DuplicateFieldName::new(struct_name, field_name)
            ));
        }
    }

    Ok(Struct::new(struct_name, fields))
}

fn typ_fun<'a>(context: Rc<FileContext<'a>>, fun: &'a parser::Fun<'a>) -> TypResult<'a, Fun<'a>> {
    let fun_name = fun.profile().name();
    let fun_typ = typ_typ(context.clone(), fun.profile().typ())?;

    let mut args_vec = vec![];

    for arg in fun.args() {
        let name = arg.name();
        let typ = typ_typ(context.clone(), arg.typ())?;
        let formal = Formal::new(name, typ.clone());

        args_vec.push(formal);
    }

    let profile = Rc::new(Formal::new(fun_name, fun_typ.clone()));

    let mut args = HashMap::new();

    for arg in &args_vec {
        if let Some(_) = args.insert(arg.name().clone(), arg.typ().clone()) {
            return Err(TypError::DuplicateArgName(DuplicateArgName::new(arg.name())));
        }
    }

    let fun_context = FunctionContext::new(
        context.clone(),
        profile.clone(),
        args,
    );

    let signature = Rc::new(Signature::new(profile.clone(), args_vec));

    if let Some(_) = context.funs().borrow_mut().insert(fun_name, signature.clone()) {
        return Err(TypError::DuplicateFunName(fun_name));
    };

    let block = typ_block(Rc::new(fun_context), fun.body())?;

    Ok(
        Fun::new(
            signature,
            // locals,
            block,
        )
    )
}

fn typ_block<'a, T>(context: Rc<T>, block: &'a parser::Block<'a>) -> TypResult<'a, Block<'a>>
    where T: ParentContext<'a> + 'a {
    let mut vars = HashMap::new();

    for var in block.vars() {
        if let Some(_) = vars.insert(var.name().clone(), typ_typ(context.context(), var.typ())?) {
            return Err(TypError::DuplicateVarName(var.name()));
        }
    }

    let new_context = Rc::new(BlockContext::new(
        context.context().clone(),
        context.clone(),
        vars,
    ));

    let mut stmts = vec![];

    for stmt in block.stmts() {
        stmts.push(typ_stmt(new_context.clone(), stmt)?)
    }

    Ok(Block::new(stmts))
}

fn typ_stmt<'a>(context: Rc<BlockContext<'a>>, stmt: &'a parser::Stmt<'a>) -> TypResult<'a, Stmt<'a>> {
    match stmt {
        parser::Stmt::SSkip => Ok(Stmt::SSkip),
        parser::Stmt::SExpr(expr) => {
            let expr = typ_expr(context, expr)?;
            Ok(Stmt::SExpr(expr))
        }
        parser::Stmt::SIf(expr, stmt_if, stmt_else) => {
            let expr = typ_expr(context.clone(), expr)?;
            let stmt_if = typ_stmt(context.clone(), stmt_if)?;
            let stmt_else = typ_stmt(context.clone(), stmt_else)?;
            Ok(Stmt::SIf(expr, Box::new(stmt_if), Box::new(stmt_else)))
        }
        parser::Stmt::SWhile(expr, stmt) => {
            let expr = typ_expr(context.clone(), expr)?;
            let stmt = typ_stmt(context.clone(), stmt)?;
            Ok(Stmt::SWhile(expr, Box::new(stmt)))
        }
        parser::Stmt::SBlock(block) => {
            // TODO add new context
            let block = typ_block(context, block)?;
            Ok(Stmt::SBlock(block))
        }
        parser::Stmt::SReturn(expr) => {
            let expr = typ_expr(context.clone(), expr)?;
            if typed_as(expr.typ(), context.profile().typ()) {
                Ok(Stmt::SReturn(expr))
            } else {
                Err(
                    TypError::WrongExpressionTyp(
                        IncompatibleTyp::new(
                            context.profile().typ().clone(),
                            expr.typ().clone(),
                        )
                    )
                )
            }
        }
    }
}

fn typ_expr<'a>(context: Rc<BlockContext<'a>>, expr: &parser::Expr<'a>) -> TypResult<'a, Expr<'a>> {
    match expr {
        parser::Expr::EConst(x) => Ok(
            Expr::new(
                ExprNode::EConst(x.clone()),
                if *x == 0 { Typ::TTypeNull } else { Typ::TInt },
            )
        ),
        parser::Expr::EVar(var_name) => {
            match context.typ(var_name) {
                None => Err(TypError::VariableDoesNotExist(var_name)),
                Some(typ) => Ok(Expr::new(
                    ExprNode::EAccessLocal(context.get_block_ident(var_name)),
                    typ.clone(),
                ))
            }
        }
        parser::Expr::EArrow(x, field_name) => {
            let expr = typ_expr(context, x.as_ref())?;
            let typ = expr.typ().clone();
            match typ {
                Typ::TStruct(structure) => {
                    match structure.fields().borrow().get(field_name) {
                        Some(field) => {
                            Ok(
                                Expr::new(
                                    ExprNode::EAccessField(Box::new(expr), field.clone()),
                                    field.clone().typ().clone(),
                                )
                            )
                        }
                        None => Err(
                            TypError::FieldDoesntExist(
                                structure.clone(),
                                field_name,
                            )
                        )
                    }
                }
                _ => Err(TypError::DereferenceNonStructPointer(field_name))
            }
        }
        parser::Expr::EAssign(expr_1, expr_2) => {
            let expr_2 = typ_expr(context.clone(), expr_2)?;
            let typ_2 = expr_2.typ();
            match expr_1.as_ref() {
                parser::Expr::EVar(var_name) => {
                    match context.typ(var_name) {
                        Some(typ_1) if typed_as(&typ_1, typ_2) => {
                            Ok(Expr::new(
                                ExprNode::EAssignLocal(context.get_block_ident(var_name), Box::new(expr_2)),
                                typ_1,
                            ))
                        }
                        _ => Err(TypError::VariableDoesNotExist(var_name))
                    }
                }
                parser::Expr::EArrow(expr, field_name) => {
                    let expr = typ_expr(context.clone(), expr)?;
                    let typ = expr.typ().clone();
                    match typ {
                        Typ::TStruct(structure) => {
                            match structure.fields().borrow().get(field_name) {
                                Some(field) if typed_as(field.typ(), typ_2) => {
                                    Ok(Expr::new(
                                        ExprNode::EAssignField(Box::new(expr), field.clone(), Box::new(expr_2)),
                                        field.typ().clone(),
                                    ))
                                }
                                _ => {
                                    Err(TypError::FieldDoesntExist(
                                        structure.clone(),
                                        field_name.clone(),
                                    ))
                                }
                            }
                        }
                        _ => {
                            Err(TypError::AccessingFieldOnNonStructTyp(
                                typ.clone(),
                                field_name.clone())
                            )
                        }
                    }
                }
                _ => Err(TypError::AssigningToNonAssignableExpression)
            }
        }
        parser::Expr::EUnop(unop, expr) => {
            let expr = typ_expr(context, expr)?;

            match unop {
                Unop::UNot => Ok(Expr::new(
                    ExprNode::EUnop(unop.clone(), Box::new(expr)),
                    Typ::TInt,
                )),
                Unop::UMinus => {
                    if typed_as(expr.typ(), &Typ::TInt) {
                        Ok(Expr::new(
                            ExprNode::EUnop(unop.clone(), Box::new(expr)),
                            Typ::TInt,
                        ))
                    } else {
                        Err(TypError::WrongExpressionTyp(
                            IncompatibleTyp::new(
                                Typ::TInt,
                                expr.typ().clone(),
                            )
                        ))
                    }
                }
            }
        }
        parser::Expr::EBinop(binop, expr_1, expr_2) => {
            let expr_1 = typ_expr(context.clone(), expr_1.as_ref())?;
            let expr_2 = typ_expr(context.clone(), expr_2.as_ref())?;
            match binop {
                Binop::BEq
                | Binop::BNeq
                | Binop::BLt
                | Binop::BGt
                | Binop::BGe
                | Binop::BLe => {
                    {
                        if typed_as(expr_1.typ(), expr_2.typ()) {
                            Ok(Expr::new(
                                ExprNode::EBinop(
                                    binop.clone(),
                                    Box::new(expr_1),
                                    Box::new(expr_2)),
                                Typ::TInt,
                            ))
                        } else {
                            Err(TypError::WrongExpressionTyp(IncompatibleTyp::new(
                                expr_1.typ().clone(),
                                expr_2.typ().clone(),
                            )))
                        }
                    }
                }
                Binop::BAdd
                | Binop::BSub
                | Binop::BMul
                | Binop::BDiv => {
                    match (typed_as(expr_1.typ(), &Typ::TInt), typed_as(expr_2.typ(), &Typ::TInt)) {
                        (true, true) => Ok(
                            Expr::new(
                                ExprNode::EBinop(
                                    binop.clone(),
                                    Box::new(expr_1),
                                    Box::new(expr_2)),
                                Typ::TInt,
                            )
                        ),
                        (false, _) => Err(TypError::WrongExpressionTyp(IncompatibleTyp::new(
                            Typ::TInt,
                            expr_1.typ().clone(),
                        ))),
                        (_, false) => Err(TypError::WrongExpressionTyp(IncompatibleTyp::new(
                            Typ::TInt,
                            expr_2.typ().clone(),
                        )))
                    }
                }
                Binop::BAnd
                | Binop::BOr => {
                    Ok(
                        Expr::new(
                            ExprNode::EBinop(
                                binop.clone(),
                                Box::new(expr_1),
                                Box::new(expr_2)),
                            Typ::TInt,
                        )
                    )
                }
            }
        }
        parser::Expr::ECall(ident, args) => {
            match ident.as_ref() {
                parser::Expr::EVar(ident) => {
                    match context.context().funs().borrow().get(ident) {
                        Some(fun) => {
                            let mut typed_args = vec![];
                            for itered in args.into_iter().zip_longest(fun.args()) {
                                match itered {
                                    EitherOrBoth::Both(arg_expr, arg_formal) => {
                                        let expr = typ_expr(context.clone(), arg_expr)?;
                                        if typed_as(arg_formal.typ(), expr.typ()) {
                                            typed_args.push(expr);
                                        } else {
                                            return Err(
                                                TypError::WrongExpressionTyp(IncompatibleTyp::new(
                                                    arg_formal.typ().clone(),
                                                    expr.typ().clone(),
                                                ))
                                            );
                                        }
                                    }
                                    EitherOrBoth::Left(_) => {
                                        return Err(
                                            TypError::TooManyArguments
                                        );
                                    }
                                    EitherOrBoth::Right(arg) => {
                                        return Err(
                                            TypError::TooFewArguments(arg.name().clone())
                                        );
                                    }
                                }
                            }
                            Ok(Expr::new(
                                ExprNode::ECall(fun.clone(), typed_args),
                                fun.profile().typ().clone(),
                            ))
                        }
                        None => {
                            Err(TypError::FunctionDoesntExist(ident))
                        }
                    }
                }
                parser::Expr::EConst(_)
                | parser::Expr::EArrow(_, _)
                | parser::Expr::EAssign(_, _)
                | parser::Expr::EUnop(_, _)
                | parser::Expr::EBinop(_, _, _)
                | parser::Expr::ECall(_, _)
                | parser::Expr::ESizeof(_) => {
                    Err(TypError::CallingANonFunctionExpression)
                }
            }
        }
        parser::Expr::ESizeof(struct_name) => {
            match context.context().structs().borrow().get(struct_name) {
                None => {
                    Err(TypError::StructDoesNotExist(struct_name))
                }
                Some(structure) => Ok(
                    Expr::new(
                        ExprNode::ESizeof(structure.clone()),
                        Typ::TInt,
                    )
                )
            }
        }
    }
}

fn typ_typ<'a>(context: Rc<FileContext<'a>>, typ: &parser::Typ<'a>) -> TypResult<'a, Typ<'a>> {
    match typ {
        parser::Typ::TInt => Ok(Typ::TInt),
        parser::Typ::TStruct(name) => {
            match context.structs().borrow().get(name) {
                None => Err(TypError::StructDoesNotExist(name)),
                Some(structure) => Ok(Typ::TStruct(structure.clone()))
            }
        }
    }
}

fn typed_as<'a>(first: &Typ<'a>, second: &Typ<'a>) -> bool {
    match (first, second) {
        (x, y) if x == y => true,
        (Typ::TTypeNull, Typ::TInt) => true,
        (Typ::TInt, Typ::TTypeNull) => true,
        (Typ::TTypeNull, Typ::TStruct(_)) => true,
        (Typ::TStruct(_), Typ::TTypeNull) => true,
        (Typ::TVoidStar, Typ::TStruct(_)) => true,
        (Typ::TStruct(_), Typ::TVoidStar) => true,
        (_, _) => false
    }
}

pub mod context {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::rc::Rc;
    use derive_getters::Getters;
    use derive_new::new;
    use crate::typer::structure::{BlockIdent, Formal, Ident, Signature, Struct, Typ};

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
                putchar.profile().name().clone(),
                Rc::new(putchar),
            );

            funs.insert(
                malloc.profile().name().clone(),
                Rc::new(malloc),
            );

            let structs = Rc::new(RefCell::new(HashMap::new()));
            let funs = Rc::new(RefCell::new(funs));

            FileContext::new(structs, funs)
        }
    }

    pub trait ParentContext<'a>: Debug {
        fn declare(&self, ident: BlockIdent<'a>, typ: Typ<'a>);
        fn typ(&self, ident: Ident<'a>) -> Option<Typ<'a>>;
        fn fresh_index(&self) -> u8;
        fn context(&self) -> Rc<FileContext<'a>>;
        fn profile(&self) -> Rc<Formal<'a>>;
        fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a>;
    }

    #[derive(Debug, Clone, Getters)]
    pub struct FunctionContext<'a> {
        context: Rc<FileContext<'a>>,
        profile: Rc<Formal<'a>>,
        block_counter: RefCell<u8>,
        arguments: RefCell<HashMap<Ident<'a>, Typ<'a>>>,
        locals: RefCell<HashMap<BlockIdent<'a>, Typ<'a>>>,
    }

    impl FunctionContext<'_> {
        const ARGUMENT_BLOCK_INDEX: u8 = 0;

        pub fn new<'a>(
            context: Rc<FileContext<'a>>,
            profile: Rc<Formal<'a>>,
            arguments: HashMap<Ident<'a>, Typ<'a>>,
        ) -> FunctionContext<'a> {
            let locals = RefCell::new(HashMap::new());

            for (name, typ) in &arguments {
                locals.borrow_mut().insert((name.clone(), FunctionContext::ARGUMENT_BLOCK_INDEX), typ.clone());
            }
            let block_counter = RefCell::new(FunctionContext::ARGUMENT_BLOCK_INDEX + 1);

            let arguments = RefCell::new(arguments);

            FunctionContext {
                context,
                profile,
                block_counter,
                arguments,
                locals,
            }
        }
    }

    impl<'a> ParentContext<'a> for FunctionContext<'a> {
        fn declare(&self, ident: BlockIdent<'a>, typ: Typ<'a>) {
            self.locals.borrow_mut().insert(ident, typ);
        }

        fn typ(&self, ident: Ident<'a>) -> Option<Typ<'a>> {
            self.arguments.borrow().get(ident).cloned()
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

        fn profile(&self) -> Rc<Formal<'a>> {
            self.profile.clone()
        }

        fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a> {
            if !self.locals().borrow().contains_key(&(ident, 0)) {
                panic!("L'argument {ident} n'existe pas : ???")
            }
            (ident, FunctionContext::ARGUMENT_BLOCK_INDEX)
            // TODO check it exists
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

            for (name, typ) in &vars {
                parent.declare((name, index), typ.clone())
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
        fn declare(&self, ident: BlockIdent<'a>, typ: Typ<'a>) {
            self.parent.declare(ident, typ)
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

        fn profile(&self) -> Rc<Formal<'a>> {
            self.parent.profile()
        }

        fn get_block_ident(&self, ident: Ident<'a>) -> BlockIdent<'a> {
            if self.vars.contains_key(ident) {
                (ident, self.index)
            } else {
                self.parent.get_block_ident(ident)
            }
        }
    }
}

#[allow(dead_code)]
pub mod error {
    use std::rc::Rc;
    use derive_new::new;
    use derive_getters::Getters;
    use crate::typer::structure::{Ident, Struct, Typ};

    #[derive(new, Debug)]
    pub struct DuplicateFieldName<'a> {
        struct_name: Ident<'a>,
        field_name: Ident<'a>,
    }

    #[derive(new, Debug, Getters)]
    pub struct IncompatibleTyp<'a> {
        expected: Typ<'a>,
        actual: Typ<'a>,
    }

    #[derive(new, Debug, Getters)]
    pub struct DuplicateArgName<'a> {
        arg_name: Ident<'a>,
    }

    #[derive(Debug)]
    pub enum TypError<'a> {
        VariableDoesNotExist(Ident<'a>),
        StructDoesNotExist(Ident<'a>),
        DuplicateVarName(Ident<'a>),
        DuplicateFunName(Ident<'a>),
        DuplicateStructName(Ident<'a>),
        DereferenceNonStructPointer(Ident<'a>),
        FieldDoesntExist(Rc<Struct<'a>>, Ident<'a>),
        AccessingFieldOnNonStructTyp(Typ<'a>, Ident<'a>),
        DuplicateFieldName(DuplicateFieldName<'a>),
        FunctionDoesntExist(Ident<'a>),
        MissingMainFunction,
        WrongMainFunctionSignature,
        TooManyArguments,
        TooFewArguments(Ident<'a>),
        CallingANonFunctionExpression,
        AssigningToNonAssignableExpression,
        WrongExpressionTyp(IncompatibleTyp<'a>),
        DuplicateArgName(DuplicateArgName<'a>),
    }
}

#[allow(dead_code)]
pub mod structure {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use derive_new::new;
    use derive_getters::Getters;

    pub type Ident<'a> = crate::parser::structure::Ident<'a>;
    pub type BlockIdent<'a> = (Ident<'a>, u8);
    pub type Const = crate::parser::structure::Const;
    pub type StructSize = usize;
    pub type Unop = crate::parser::structure::Unop;
    pub type Binop = crate::parser::structure::Binop;

    #[derive(new, Debug, Getters)]
    pub struct File<'a> {
        funs: HashMap<Ident<'a>, Fun<'a>>,
    }

    impl<'a> File<'a> {
        pub fn into_funs(self) -> HashMap<Ident<'a>, Fun<'a>> {
            self.funs
        }
    }

    #[derive(new, Debug, Getters)]
    pub struct Signature<'a> {
        profile: Rc<Formal<'a>>,
        args: Vec<Formal<'a>>,
    }

    #[derive(new, Debug, Getters)]
    pub struct Fun<'a> {
        signature: Rc<Signature<'a>>,
        // locals: Vec<BlockIdent<'a>>,
        block: Block<'a>,
    }

    #[derive(new, Debug, Getters)]
    pub struct Formal<'a> {
        name: Ident<'a>,
        typ: Typ<'a>,
    }

    #[derive(new, Debug, Getters)]
    pub struct Field<'a> {
        name: Ident<'a>,
        typ: Typ<'a>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Typ<'a> {
        TInt,
        TVoidStar,
        TTypeNull,
        TStruct(Rc<Struct<'a>>),
    }

    #[derive(new, Debug, Getters)]
    pub struct Struct<'a> {
        name: Ident<'a>,
        fields: Rc<RefCell<HashMap<Ident<'a>, Rc<Field<'a>>>>>, // TODO remove refcell
    }

    impl Struct<'_> {
        const FIELD_SIZE: usize = 8;

        pub fn c_size<'a>(&self) -> Const {
            (self.fields.borrow().len() * Struct::FIELD_SIZE) as Const
        }
    }

    #[derive(new, Debug, Getters)]
    pub struct Block<'a> {
        stmts: Vec<Stmt<'a>>,
    }

    #[derive(Debug)]
    pub enum Stmt<'a> {
        SSkip,
        SExpr(Expr<'a>),
        SIf(Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
        SWhile(Expr<'a>, Box<Stmt<'a>>),
        SBlock(Block<'a>),
        SReturn(Expr<'a>),
    }

    #[derive(new, Debug, Getters)]
    pub struct Expr<'a> {
        node: ExprNode<'a>,
        typ: Typ<'a>,
    }

    #[derive(Debug)]
    pub enum ExprNode<'a> {
        EConst(Const),
        EAccessLocal(BlockIdent<'a>),
        EAccessField(Box<Expr<'a>>, Rc<Field<'a>>),
        EAssignLocal(BlockIdent<'a>, Box<Expr<'a>>),
        EAssignField(Box<Expr<'a>>, Rc<Field<'a>>, Box<Expr<'a>>),
        EUnop(Unop, Box<Expr<'a>>),
        EBinop(Binop, Box<Expr<'a>>, Box<Expr<'a>>),
        ECall(Rc<Signature<'a>>, Vec<Expr<'a>>),
        ESizeof(Rc<Struct<'a>>),
    }

    impl PartialEq<Self> for Struct<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Signature<'_> {
        pub const MAIN_NAME: &'static str = "main";
        pub const PUTCHAR_NAME: &'static str = "putchar";
        pub const MALLOC_NAME: &'static str = "malloc";

        pub fn main<'a>() -> Signature<'a> {
            Signature::new(Rc::new(Formal::new(Signature::MAIN_NAME, Typ::TInt)), vec![])
        }

        pub fn putchar<'a>() -> Signature<'a> {
            Signature::new(Rc::new(Formal::new(Signature::PUTCHAR_NAME, Typ::TInt)), vec![Formal::new("c", Typ::TInt)])
        }

        pub fn malloc<'a>() -> Signature<'a> {
            Signature::new(Rc::new(Formal::new(Signature::MALLOC_NAME, Typ::TVoidStar)), vec![Formal::new("n", Typ::TInt)])
        }
    }
}