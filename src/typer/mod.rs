pub mod context;
pub mod error;
pub mod structure;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use itertools::{EitherOrBoth, enumerate, Itertools};
use structure::*;
use crate::parser::structure as parser;
use crate::typer::context::{BlockContext, FileContext, FunctionContext, ParentContext};
use crate::typer::error::{DuplicateFieldName, IncompatibleTyp, TypError};

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
            if signature.typ() != &Typ::TInt {
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

    for (index, field) in enumerate(structure.fields()) {
        let field_name = field.name();

        if let Some(_) = fields.borrow_mut().insert(
            field_name.clone(),
            Rc::new(Field::new(
                field_name,
                index,
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

    let mut arg_names = HashSet::new();
    let mut args_vec = vec![];

    for (index, arg) in enumerate(fun.args()) {
        let name = arg.name();

        if !arg_names.insert(name.clone()) {
            return Err(TypError::DuplicateArgName(name.clone()));
        }

        let block_name = BlockIdent::Arg(index, name.clone());
        let typ = typ_typ(context.clone(), arg.typ())?;
        let formal = Formal::new(block_name.clone(), typ.clone());

        // TODO check arg names
        args_vec.push(formal);
    }

    let fun_context = Rc::new(FunctionContext::new(
        context.clone(),
        fun_name.clone(),
        fun_typ.clone(),
        args_vec.clone(),
    ));

    let signature = Rc::new(Signature::new(fun_name.clone(), fun_typ.clone(), args_vec));

    if let Some(_) = context.funs().borrow_mut().insert(fun_name.clone(), signature.clone()) {
        return Err(TypError::DuplicateFunName(fun_name));
    };

    let block = typ_block(fun_context.clone(), fun.body())?;

    let locals = fun_context.locals().borrow().clone();

    Ok(
        Fun::new(
            signature,
            locals,
            block,
        )
    )
}

fn typ_block<'a, T>(context: Rc<T>, block: &'a parser::Block<'a>) -> TypResult<'a, Block<'a>>
    where T: ParentContext<'a> + 'a {
    let mut vars = HashMap::new();

    for var in block.vars() {
        if let Some(_) = vars.insert(var.name().clone(), typ_typ(context.context(), var.typ())?) {
            return Err(TypError::DuplicateVarName(var.name().clone()));
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
            if typed_as(expr.typ(), &context.fun_typ()) {
                Ok(Stmt::SReturn(expr))
            } else {
                Err(
                    TypError::WrongExpressionTyp(
                        IncompatibleTyp::new(
                            context.fun_typ(),
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
                None => Err(TypError::VariableDoesNotExist),
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
                        _ => Err(TypError::VariableDoesNotExist)
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
                                            typed_args.push(ArgExpr::new(
                                                arg_formal.clone(),
                                                expr,
                                            ));
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
                                    EitherOrBoth::Right(_) => {
                                        return Err(
                                            // TODO args to error
                                            TypError::TooFewArguments
                                        );
                                    }
                                }
                            }
                            Ok(Expr::new(
                                ExprNode::ECall(fun.clone(), typed_args),
                                fun.typ().clone(),
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
                        ExprNode::EConst(structure.c_size()),
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
