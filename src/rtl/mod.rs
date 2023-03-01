pub mod error;

pub mod structure;

use std::collections::HashMap;
use crate::rtl::error::RtlError;
use crate::rtl::structure::{File, Fresh, Fun, Instr, Mbinop, MuBranch, Munop};
use crate::rtl::structure::graph::Graph;
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;
use crate::typer::structure as typer;
use crate::typer::structure::Binop;

type RtlResult<T> = Result<T, RtlError>;

pub fn rtl_file(file: &typer::File) -> RtlResult<File> {
    let mut funs = vec![];

    for (_, fun) in file.funs() {
        funs.push(rtl_fun(fun)?);
    }

    Ok(File::new(funs))
}

fn rtl_fun(fun: &typer::Fun) -> RtlResult<Fun> {
    let name = String::from(fun.signature().name().clone());
    let result = Register::fresh();

    let mut vars = HashMap::new();

    for local in fun.locals() {
        let register = Register::fresh();

        match vars.insert(local.clone().into(), register) {
            None => {}
            Some(_) => {
                return Err(RtlError::Any(format!("Duplicate block name : {:?}", local)));
            }
        }
    }

    let graph = Graph::new(vars);

    let exit = Label::fresh();
    let entry = rtl_block(
        &graph,
        &result,
        &exit,
        &exit,
        fun.block(),
    )?;

    Ok(Fun::new(
        name,
        result,
        entry,
        exit,
        graph,
    ))
}

fn rtl_block(graph: &Graph, retr: &Register, retl: &Label, destl: &Label, block: &typer::Block) -> RtlResult<Label> {
    let mut dlabel = destl.clone();

    for stmt in block.stmts().iter().rev() {
        dlabel = rtl_stmt(graph, retr, retl, &dlabel, stmt)?;
    }

    Ok(dlabel.clone())
}

fn rtl_stmt(graph: &Graph, retr: &Register, retl: &Label, destl: &Label, stmt: &typer::Stmt) -> RtlResult<Label> {
    match stmt {
        typer::Stmt::SSkip => Ok(destl.clone()),
        typer::Stmt::SExpr(expr) => rtl_expr(
            graph,
            &Register::fresh(),
            destl,
            expr,
        ),
        typer::Stmt::SIf(expr, stmt_if, stmt_else) => {
            let to_else = rtl_stmt(graph, retr, retl, destl, stmt_else)?;
            let to_if = rtl_stmt(graph, retr, retl, destl, stmt_if)?;
            let condition_register = Register::fresh();
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
            let expr_reg = Register::fresh();
            let expr_cond_label = graph.insert(Instr::EMuBranch(MuBranch::MJz, expr_reg.clone(), destl.clone(), stmt_label));
            let expr_label = rtl_expr(graph, &expr_reg, &expr_cond_label, expr)?;
            graph.insert_with_label(goto_label, Instr::EGoto(expr_label.clone()));
            Ok(expr_label)
        }
        typer::Stmt::SBlock(block) => rtl_block(graph, retr, retl, destl, block),
        typer::Stmt::SReturn(expr) => rtl_expr(graph, retr, retl, expr)
    }
}

fn rtl_expr(graph: &Graph, destr: &Register, destl: &Label, expr: &typer::Expr) -> RtlResult<Label> {
    match expr.node() {
        typer::ExprNode::EConst(x) => Ok(
            graph.insert(Instr::EConst(x.clone(), destr.clone(), destl.clone()))
        ),
        typer::ExprNode::EAccessLocal(var) => {
            let register = graph.vars().borrow().get(&(var.clone().into())).expect("Failed to find block_ident").clone();
            Ok(graph.insert(Instr::EMBinop(Mbinop::MMov,
                                           register,
                                           destr.clone(),
                                           destl.clone(),
            )))
        }
        typer::ExprNode::EAccessField(_, _) => todo!(),
        typer::ExprNode::EAssignLocal(var, expr) => {
            let expr_reg = graph.vars().borrow().get(&var.clone().into()).expect("Register not found").clone();
            let mov_lbl = graph.insert(Instr::EMBinop(Mbinop::MMov, expr_reg.clone(), destr.clone(), destl.clone()));
            rtl_expr(graph, &expr_reg, &mov_lbl, expr)
        }
        typer::ExprNode::EAssignField(_, _, _) => todo!(),
        typer::ExprNode::EUnop(unop, expr) => {
            match unop {
                typer::Unop::UNot => {
                    let expr_reg = Register::fresh();
                    let test_lbl = graph.insert(Instr::EMUnop(Munop::Msetei(0), expr_reg.clone(), destl.clone()));
                    rtl_expr(graph, &expr_reg, &test_lbl, expr)
                }
                typer::Unop::UMinus => {
                    let expr_reg = Register::fresh();
                    let sub_lbl = graph.insert(Instr::EMBinop(Mbinop::MSub, expr_reg.clone(), destr.clone(), destl.clone()));
                    let zero_lbl = graph.insert(Instr::EConst(0, destr.clone(), sub_lbl));
                    rtl_expr(graph, &expr_reg, &zero_lbl, expr)
                }
            }
        }
        typer::ExprNode::EBinop(op, expr_1, expr_2) => {
            match op {
                Binop::BAdd
                | Binop::BSub
                | Binop::BMul
                | Binop::BDiv
                | Binop::BEq
                | Binop::BNeq
                | Binop::BLt
                | Binop::BGt
                | Binop::BGe
                | Binop::BLe => {
                    let rtl_op = match op {
                        Binop::BAdd => Mbinop::MAdd,
                        Binop::BSub => Mbinop::MSub,
                        Binop::BMul => Mbinop::MMul,
                        Binop::BDiv => Mbinop::MDiv,
                        Binop::BEq => Mbinop::MSete,
                        Binop::BNeq => Mbinop::MSetne,
                        Binop::BLt => Mbinop::Msetl,
                        Binop::BGt => Mbinop::Msetg,
                        Binop::BGe => Mbinop::Msetge,
                        Binop::BLe => Mbinop::Msetle,
                        _ => unreachable!()
                    };
                    let reg_2 = Register::fresh();
                    let operation_lbl = graph.insert(Instr::EMBinop(
                        rtl_op,
                        reg_2.clone(),
                        destr.clone(),
                        destl.clone(),
                    ));
                    let expr_2_lbl = rtl_expr(graph, &reg_2, &operation_lbl, expr_2)?;
                    rtl_expr(graph, &destr, &expr_2_lbl, expr_1)
                }
                Binop::BAnd => todo!(),
                Binop::BOr => todo!()
            }
        }
        typer::ExprNode::ECall(signature, args) => {
            let eval_label = Label::fresh();

            let mut arg_label = eval_label.clone();
            let mut reverse_args = vec![];

            for arg in args.iter().rev() {
                let reg = Register::fresh();
                reverse_args.push(reg.clone());
                arg_label = rtl_expr(graph, &reg, &arg_label, arg.expr())?;
            }

            reverse_args.reverse();

            graph.insert_with_label(eval_label.clone(), Instr::ECall(
                destr.clone(),
                String::from(signature.name().clone()),
                reverse_args,
                destl.clone(),
            ));

            Ok(arg_label)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::rc::Rc;
    use crate::parser::{Input, parse_file};
    use crate::rtl::rtl_file;
    use crate::typer::context::FileContext;
    use crate::typer::typ_file;

    #[test]
    fn display() {
        let content = read_to_string("test.c").unwrap();
        let (_, file) = parse_file(Input::new(&content)).unwrap();
        let typed = typ_file(Rc::new(FileContext::default()), &file).unwrap();
        let rtl = rtl_file(&typed).unwrap();

        println!("{}", rtl);
    }
}


