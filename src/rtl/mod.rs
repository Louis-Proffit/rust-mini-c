pub mod error;
pub mod interpreter;
pub mod structure;

use std::collections::HashMap;
use crate::rtl::error::RtlError;
use crate::rtl::structure::{BlockIdent, File, Fresh, Fun, Instr, Mbinop, MuBranch, Munop};
use crate::rtl::structure::graph::Graph;
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::PseudoRegister;
use crate::typer::structure as typer;

pub type RtlResult<'a, T> = Result<T, RtlError<'a>>;

pub fn rtl_file<'a>(file: &typer::File<'a>) -> RtlResult<'a, File<'a>> {
    let mut funs = HashMap::new();

    for (name, fun) in file.funs() {
        funs.insert(name.clone(), rtl_fun(fun)?);
    }

    Ok(File::new(funs))
}

fn rtl_fun<'a>(fun: &typer::Fun<'a>) -> RtlResult<'a, Fun<'a>> {
    let name = fun.signature().name().clone();
    let result = PseudoRegister::fresh();

    let mut arguments = vec![];
    let mut locals = HashMap::new();
    let mut vars = HashMap::new();

    for argument in fun.signature().args() {
        let register = PseudoRegister::fresh();
        arguments.push(register.clone());
        let argument_name:  BlockIdent = argument.name().clone().into();
        match vars.insert(argument_name.clone(), register) {
            None => {}
            Some(_) => {
                return Err(RtlError::DuplicateBlockIdent(argument_name));
            }
        }
    }

    for local in fun.locals() {
        let local:BlockIdent = local.clone().into();
        let register = PseudoRegister::fresh();
        locals.insert(local.clone(), register.clone());
        match vars.insert(local.clone(), register) {
            None => {}
            Some(_) => {
                return Err(RtlError::DuplicateBlockIdent(local.clone()));
            }
        }
    }

    let mut graph = Graph::new(vars);

    let exit = Label::fresh();
    let entry = rtl_block(
        &mut graph,
        &result,
        &exit,
        &exit,
        fun.block(),
    )?;

    Ok(Fun::new(
        name,
        result,
        arguments,
        locals,
        entry,
        exit,
        graph,
    ))
}

fn rtl_block<'a>(graph: &mut Graph<'a>, retr: &PseudoRegister, retl: &Label, destl: &Label, block: &typer::Block<'a>) -> RtlResult<'a, Label> {
    let mut dlabel = destl.clone();

    for stmt in block.stmts().iter().rev() {
        dlabel = rtl_stmt(graph, retr, retl, &dlabel, stmt)?;
    }

    Ok(dlabel.clone())
}

fn rtl_stmt<'a>(graph: &mut Graph<'a>, retr: &PseudoRegister, retl: &Label, destl: &Label, stmt: &typer::Stmt<'a>) -> RtlResult<'a, Label> {
    match stmt {
        typer::Stmt::SSkip => Ok(destl.clone()),
        typer::Stmt::SExpr(expr) => rtl_expr(
            graph,
            &PseudoRegister::fresh(),
            destl,
            expr,
        ),
        typer::Stmt::SIf(expr, stmt_if, stmt_else) => {
            let to_else = rtl_stmt(graph, retr, retl, destl, stmt_else)?;
            let to_if = rtl_stmt(graph, retr, retl, destl, stmt_if)?;
            let condition_register = PseudoRegister::fresh();
            let to_jmp = graph.insert(Instr::EMuBranch(
                MuBranch::MJnz,
                condition_register.clone(),
                to_if,
                to_else,
            ));
            rtl_expr(graph,
                     &condition_register,
                     &to_jmp,
                     expr,
            )
        }
        typer::Stmt::SWhile(expr, stmt) => {
            let goto_label = Label::fresh();
            let stmt_label = rtl_stmt(graph, retr, retl, &goto_label, stmt)?;
            let expr_reg = PseudoRegister::fresh();
            let expr_cond_label = graph.insert(Instr::EMuBranch(MuBranch::MJz, expr_reg.clone(), destl.clone(), stmt_label));
            let expr_label = rtl_expr(graph, &expr_reg, &expr_cond_label, expr)?;
            graph.insert_with_label(goto_label, Instr::EGoto(expr_label.clone()));
            Ok(expr_label)
        }
        typer::Stmt::SBlock(block) => rtl_block(graph, retr, retl, destl, block),
        typer::Stmt::SReturn(expr) => rtl_expr(graph, retr, retl, expr)
    }
}

fn rtl_expr<'a>(graph: &mut Graph<'a>, destr: &PseudoRegister, destl: &Label, expr: &typer::Expr<'a>) -> RtlResult<'a, Label> {
    match expr.node() {
        typer::ExprNode::EConst(x) => Ok(
            graph.insert(Instr::EConst(x.clone(), destr.clone(), destl.clone()))
        ),
        typer::ExprNode::EAccessLocal(var) => {
            let var = var.clone().into();
            let register = graph
                .vars
                .get(&var)
                .ok_or(RtlError::VarNotFound(var))?
                .clone();
            Ok(graph.insert(Instr::EMBinop(Mbinop::MMov,
                                           register,
                                           destr.clone(),
                                           destl.clone(),
            )))
        }
        typer::ExprNode::EAccessField(expr, y) => {
            let expr_reg = PseudoRegister::fresh();
            let field_lbl = graph.insert(Instr::ELoad(expr_reg.clone(), y.c_offset(), destr.clone(), destl.clone()));
            rtl_expr(graph, &expr_reg, &field_lbl, expr)
        }
        typer::ExprNode::EAssignLocal(var, expr) => {
            let expr_reg = graph.vars.get(&var.clone().into()).expect("Register not found").clone();
            let mov_lbl = graph.insert(Instr::EMBinop(Mbinop::MMov, expr_reg.clone(), destr.clone(), destl.clone()));
            rtl_expr(graph, &expr_reg, &mov_lbl, expr)
        }
        typer::ExprNode::EAssignField(expr, field, value) => {
            let expr_reg = PseudoRegister::fresh();
            let store_lbl = graph.insert(Instr::EStore(destr.clone(), expr_reg.clone(), field.c_offset(), destl.clone()));
            let expr_lbl = rtl_expr(graph, &expr_reg, &store_lbl, expr)?;
            rtl_expr(graph, &destr, &expr_lbl, value)
        }
        typer::ExprNode::EUnop(unop, expr) => {
            match unop {
                typer::Unop::UNot => {
                    let test_lbl = graph.insert(Instr::EMUnop(Munop::Msetei(0), destr.clone(), destl.clone()));
                    rtl_expr(graph, &destr, &test_lbl, expr)
                }
                typer::Unop::UMinus => {
                    let expr_reg = PseudoRegister::fresh();
                    let sub_lbl = graph.insert(Instr::EMBinop(Mbinop::MSub, expr_reg.clone(), destr.clone(), destl.clone()));
                    let zero_lbl = graph.insert(Instr::EConst(0, destr.clone(), sub_lbl));
                    rtl_expr(graph, &expr_reg, &zero_lbl, expr)
                }
            }
        }
        typer::ExprNode::EBinop(op, expr_1, expr_2) => {
            match op {
                typer::Binop::BAdd
                | typer::Binop::BSub
                | typer::Binop::BMul
                | typer::Binop::BDiv
                | typer::Binop::BEq
                | typer::Binop::BNeq
                | typer::Binop::BLt
                | typer::Binop::BGt
                | typer::Binop::BGe
                | typer::Binop::BLe => {
                    let rtl_op = match op {
                        typer::Binop::BAdd => Mbinop::MAdd,
                        typer::Binop::BSub => Mbinop::MSub,
                        typer::Binop::BMul => Mbinop::MMul,
                        typer::Binop::BDiv => Mbinop::MDiv,
                        typer::Binop::BEq => Mbinop::MSete,
                        typer::Binop::BNeq => Mbinop::MSetne,
                        typer::Binop::BLt => Mbinop::Msetl,
                        typer::Binop::BGt => Mbinop::Msetg,
                        typer::Binop::BGe => Mbinop::Msetge,
                        typer::Binop::BLe => Mbinop::Msetle,
                        _ => unreachable!()
                    };
                    let reg_2 = PseudoRegister::fresh();
                    let operation_lbl = graph.insert(Instr::EMBinop(
                        rtl_op,
                        reg_2.clone(),
                        destr.clone(),
                        destl.clone(),
                    ));
                    let expr_2_lbl = rtl_expr(graph, &reg_2, &operation_lbl, expr_2)?;
                    rtl_expr(graph, &destr, &expr_2_lbl, expr_1)
                }
                typer::Binop::BAnd => {
                    let set_1_lbl = graph.insert(Instr::EConst(1, destr.clone(), destl.clone()));
                    let cmp_2_label = graph.insert(Instr::EMuBranch(MuBranch::MJz, destr.clone(), destl.clone(), set_1_lbl));
                    let expr_2_lbl = rtl_expr(graph, destr, &cmp_2_label, expr_2)?;
                    let cmp_1_label = graph.insert(Instr::EMuBranch(MuBranch::MJz, destr.clone(), destl.clone(), expr_2_lbl));
                    rtl_expr(graph, destr, &cmp_1_label, expr_1)
                }
                typer::Binop::BOr => {
                    let set_1_lbl = graph.insert(Instr::EConst(1, destr.clone(), destl.clone()));
                    let cmp_2_label = graph.insert(Instr::EMuBranch(MuBranch::MJz, destr.clone(), destl.clone(), set_1_lbl.clone()));
                    let expr_2_lbl = rtl_expr(graph, destr, &cmp_2_label, expr_2)?;
                    let cmp_1_label = graph.insert(Instr::EMuBranch(MuBranch::MJnz, destr.clone(), set_1_lbl.clone(), expr_2_lbl));
                    rtl_expr(graph, destr, &cmp_1_label, expr_1)
                }
            }
        }
        typer::ExprNode::ECall(signature, args) => {
            let eval_label = Label::fresh();

            let mut arg_label = eval_label.clone();
            let mut reverse_args = vec![];

            for arg in args.iter().rev() {
                let reg = PseudoRegister::fresh();
                reverse_args.push(reg.clone());
                arg_label = rtl_expr(graph, &reg, &arg_label, arg.expr())?;
            }

            reverse_args.reverse();

            graph.insert_with_label(eval_label.clone(), Instr::ECall(
                destr.clone(),
                signature.name().clone(),
                reverse_args,
                destl.clone(),
            ));

            Ok(arg_label)
        }
    }
}


