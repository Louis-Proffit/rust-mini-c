use std::rc::Rc;
use derive_new::new;
use derive_getters::Getters;
use crate::common::Ident;
use crate::typer::structure::{BlockIdent, Struct, Typ};

#[allow(dead_code)]
#[derive(new, Debug)]
pub struct DuplicateFieldName<'a> {
    struct_name: Ident<'a>,
    field_name: Ident<'a>,
}

#[allow(dead_code)]
#[derive(new, Debug, Getters)]
pub struct IncompatibleTyp<'a> {
    expected: Typ<'a>,
    actual: Typ<'a>,
}

#[allow(dead_code)]
#[derive(new, Debug, Getters)]
pub struct DuplicateArgName<'a> {
    arg_name: BlockIdent<'a>,
}

#[derive(Debug)]
pub enum TypError<'a> {
    VariableDoesNotExist,
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
    TooFewArguments,
    CallingANonFunctionExpression,
    AssigningToNonAssignableExpression,
    WrongExpressionTyp(IncompatibleTyp<'a>),
    DuplicateArgName(Ident<'a>),
}
