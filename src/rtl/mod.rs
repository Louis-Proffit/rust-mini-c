pub mod error;

pub mod structure;

use std::collections::HashMap;
use crate::rtl::error::RtlError;
use crate::rtl::structure::{File, Fresh, Fun, Instr, Mbinop, MuBranch, Munop};
use crate::rtl::structure::graph::Graph;
use crate::rtl::structure::label::Label;
use crate::rtl::structure::register::Register;
use crate::typer::structure as typer;

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

        vars.insert(local.clone().into(), register).expect("Duplicate block_index");
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
        typer::Stmt::SReturn(expr) => rtl_expr(
            graph,
            retr,
            retl,
            expr,
        )
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
        typer::ExprNode::EAssignLocal(_, _) => todo!(),
        typer::ExprNode::EAssignField(_, _, _) => todo!(),
        typer::ExprNode::EUnop(unop, expr) => {
            match unop {
                typer::Unop::UNot => {
                    let expr_reg = Register::fresh();
                    let test_lbl = graph.insert(Instr::EMUnop(Munop::Msetei(0), expr_reg, destl.clone()));
                    rtl_expr(graph, &expr_reg, &test_lbl, expr)
                }
                typer::Unop::UMinus => {
                    let comp_label = graph.insert(Instr::EMUnop(Munop::Mneg, destr.clone(), destl.clone()));
                    rtl_expr(graph, destr, &comp_label, expr)
                }
            }
        }
        typer::ExprNode::EBinop(_, _, _) => todo!(),
        typer::ExprNode::ECall(_, _) => todo!()
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


