use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mini-c.pest"]
struct MiniCParser;

#[allow(dead_code)]
pub mod structure {
    pub type Ident<'a> = &'a str;
    pub type Const = i32;

    #[derive(Debug)]
    pub struct File<'a> {
        pub funs: Vec<Fun<'a>>,
        pub structs: Vec<Struct<'a>>,
    }

    #[derive(Debug)]
    pub enum Typ<'a> {
        TInt,
        TStruct(Ident<'a>),
    }

    #[derive(Debug)]
    pub struct Formal<'a> {
        pub(crate) name: Ident<'a>,
        pub(crate) typ: Typ<'a>,
    }

    #[derive(Debug)]
    pub struct Struct<'a> {
        pub(crate) name: Ident<'a>,
        pub(crate) fields: Vec<Formal<'a>>,
    }

    #[derive(Debug)]
    pub struct Fun<'a> {
        pub profile: Formal<'a>,
        pub(crate) args: Vec<Formal<'a>>,
        pub(crate) body: Block<'a>,
    }

    #[derive(Debug)]
    pub struct Block<'a> {
        pub(crate) vars: Vec<Formal<'a>>,
        pub(crate) stmts: Vec<Stmt<'a>>,
    }

    #[derive(Debug)]
    pub enum Stmt<'a> {
        SSkip,
        SExpr(Expr<'a>),
        SIf(Expr<'a>, Box<Stmt<'a>>, Option<Box<Stmt<'a>>>),
        SWhile(Expr<'a>, Box<Stmt<'a>>),
        SBlock(Block<'a>),
        SReturn(Expr<'a>),
    }

    #[derive(Debug)]
    pub enum Expr<'a> {
        EConst(Const),
        EIdent(Ident<'a>),
        EArrow(Box<Expr<'a>>, Ident<'a>),
        EAssign(Box<Expr<'a>>, Box<Expr<'a>>),
        EUnop(Unop, Box<Expr<'a>>),
        EBinop(Binop, Box<Expr<'a>>, Box<Expr<'a>>),
        ECall(Box<Expr<'a>>, Vec<Expr<'a>>),
        ESizeof(Ident<'a>),
    }

    #[derive(Debug)]
    pub enum Unop {
        UNot,
        UMinus,
    }

    #[derive(Debug)]
    pub enum Binop {
        BEq,
        BNeq,
        BLt,
        BGt,
        BGe,
        BLe,
        BAdd,
        BSub,
        BMul,
        BDiv,
        BAnd,
        BOr,
    }
}

pub mod raw_nom {
    use crate::parser::structure::*;
    use nom::branch::alt;
    use nom::combinator::{map, opt};
    use nom::IResult;
    use nom::multi::{many0, separated_list1};
    use logos_nom_bridge::{token_parser, Tokens};
    use nom::Err::Error;
    use nom::error::ErrorKind;
    use nom::sequence::tuple;
    use Token::Semicolon;
    use crate::lexer::Token;
    use crate::parser::structure::Expr::EBinop;

    pub type Input<'src> = Tokens<'src, Token>;

    token_parser!(token: Token);

    pub fn parse_file(input: Input) -> IResult<Input, File> {
        let mut structs = vec![];
        let mut funs = vec![];
        let struct_parser = map(decl_struct, |x| structs.push(x));
        let fun_parser = map(decl_fun, |x| funs.push(x));
        let (input, _) = many0(alt((struct_parser, fun_parser)))(input)?;
        Ok((input, File { structs, funs }))
    }

    fn decl_fun(input: Input) -> IResult<Input, Fun> {
        map(
            tuple((formal, Token::Lpar, many0(formal), Token::Rpar, block)),
            |(profile, _, args, _, body)| Fun { profile, args, body },
        )(input)
    }

    fn decl_struct(input: Input) -> IResult<Input, Struct> {
        map(
            tuple((Token::Struct, ident, Token::Lbrace, many0(decl_var), Token::Rbrace, Semicolon)),
            |(_, name, _, fields, _, _)| Struct { name, fields: fields.into_iter().flatten().collect() },
        )(input)
    }

    fn star_ident(input: Input) -> IResult<Input, Ident> {
        map(tuple((Token::Star, ident)), |(_, x)| x)(input)
    }

    fn decl_var(input: Input) -> IResult<Input, Vec<Formal>> {
        let int_decl_var = tuple((Token::Int, separated_list1(Token::Comma, ident), Semicolon));
        let struct_decl_var = tuple((Token::Int, ident, separated_list1(Token::Comma, star_ident), Semicolon));
        alt((
            map(int_decl_var, |(_, idents, _)| idents.into_iter().map(|ident| Formal { name: ident, typ: Typ::TInt }).collect()),
            map(struct_decl_var, |(_, struct_ident, idents, _)| idents.into_iter().map(|ident| Formal { name: ident, typ: Typ::TStruct(struct_ident) }).collect()),
        ))(input)
    }

    fn formal(input: Input) -> IResult<Input, Formal> {
        alt((
            map(tuple((Token::Int, ident)), |(_, ident)| Formal { typ: Typ::TInt, name: ident }),
            map(tuple((Token::Struct, ident, Token::Star, ident)), |(_, struct_ident, _, ident)| Formal { typ: Typ::TStruct(struct_ident), name: ident })
        ))(input)
    }

    fn ident(input: Input) -> IResult<Input, Ident> {
        match input.peek() {
            Some((Token::Ident, x)) => Ok((input.advance(), x)),
            _ => Err(
                Error(
                    nom::error::Error::new(input, ErrorKind::IsA),
                ),
            )
        }
    }

    fn block(input: Input) -> IResult<Input, Block> {
        let mut vars = vec![];
        let mut stmts = vec![];
        let (input, _) = tuple((Token::Lbrace, many0(alt((
            map(decl_var, |mut x| vars.append(&mut x)),
            map(stmt, |x| stmts.push(x))
        )))))(input)?;
        Ok((input, Block { vars, stmts }))
    }

    fn stmt(input: Input) -> IResult<Input, Stmt> {
        let expr_stmt = tuple((opt(expr), Semicolon));
        let selection_stmt = tuple((Token::If, Token::Lpar, expr, Token::Rpar, stmt, opt(tuple((Token::Else, stmt)))));
        let iteration_stmt = tuple((Token::While, Token::Lpar, expr, Token::Rpar, stmt));
        let jump_stmt = tuple((Token::Return, opt(expr), Semicolon));

        alt((
            map(
                block,
                |x| Stmt::SBlock(x),
            ),
            map(
                expr_stmt,
                |(expr, _)| expr.map_or(Stmt::SSkip, |expr| Stmt::SExpr(expr)),
            ),
            map(
                selection_stmt,
                |(_, _, expr, _, e_if, e_else)| Stmt::SIf(expr, Box::new(e_if), e_else.map(|(_, stmt)| Box::new(stmt))),
            ),
            map(
                iteration_stmt,
                |(_, _, expr, _, stmt)| Stmt::SWhile(expr, Box::new(stmt)),
            ),
            map(
                jump_stmt,
                |(_, expr, _)| expr.map_or(Stmt::SSkip, |expr| Stmt::SReturn(expr)),
            )
        ))(input)
    }

    fn expr(input: Input) -> IResult<Input, Expr> {
        assign_expr(input)
    }

    fn primary_expr(input: Input) -> IResult<Input, Expr> {
        alt((
            map(integer, |x| Expr::EConst(x)),
            map(ident, |x| Expr::EIdent(x)),
            map(tuple((Token::Lpar, expr, Token::Rpar)), |(_, x, _)| x),
        ))(input)
    }

    fn sizeof_expr(input: Input) -> IResult<Input, Expr> {
        map(
            tuple((Token::Sizeof, Token::Lpar, Token::Struct, ident, Token::Rpar)),
            |(_, _, _, ident, _)| Expr::ESizeof(ident),
        )(input)
    }

    fn postfix_expression(input: Input) -> IResult<Input, Expr> {
        enum PostfixSuffix<'a> {
            Call(Vec<Expr<'a>>),
            Access(Ident<'a>),
        }

        fn postfix_suffix(input: Input) -> IResult<Input, PostfixSuffix> {
            alt((
                map(
                    tuple((Token::Lpar, many0(assign_expr), Token::Rpar)),
                    |(_, exprs, _)| PostfixSuffix::Call(exprs),
                ),
                map(
                    tuple((Token::Arrow, ident)),
                    |(_, ident)| PostfixSuffix::Access(ident),
                )
            ))(input)
        }

        let (input, (mut expr, suffixes)) = tuple((primary_expr, many0(postfix_suffix)))(input)?;

        for suffix in suffixes {
            match suffix {
                PostfixSuffix::Call(args) => expr = Expr::ECall(Box::new(expr), args),
                PostfixSuffix::Access(ident) => expr = Expr::EArrow(Box::new(expr), ident)
            }
        }

        Ok((input, expr))
    }

    fn unary_op(input: Input) -> IResult<Input, Unop> {
        alt((
            map(Token::Minus, |_| Unop::UMinus),
            map(Token::Bang, |_| Unop::UNot),
        ))(input)
    }

    fn unary_expr(input: Input) -> IResult<Input, Expr> {
        alt((
            postfix_expression,
            map(tuple((unary_op, unary_expr)), |(op, expr)| Expr::EUnop(op, Box::new(expr))),
            sizeof_expr
        ))(input)
    }

    fn multiplicative_op(input: Input) -> IResult<Input, Binop> {
        alt((
            map(Token::Star, |_| Binop::BMul),
            map(Token::Div, |_| Binop::BDiv),
        ))(input)
    }

    fn additive_op(input: Input) -> IResult<Input, Binop> {
        alt((
            map(Token::Plus, |_| Binop::BSub),
            map(Token::Minus, |_| Binop::BAdd),
        ))(input)
    }

    fn relation_op(input: Input) -> IResult<Input, Binop> {
        alt((
            map(Token::Le, |_| Binop::BLe),
            map(Token::Lt, |_| Binop::BLt),
            map(Token::Ge, |_| Binop::BGe),
            map(Token::Gt, |_| Binop::BGt),
        ))(input)
    }

    fn eq_op(input: Input) -> IResult<Input, Binop> {
        alt((
            map(Token::Eq, |_| Binop::BEq),
            map(Token::BangEq, |_| Binop::BNeq),
        ))(input)
    }

    fn multiplicative_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((unary_expr, many0(tuple((multiplicative_op, unary_expr)))))(input)?;

        for (binop, other_expr) in other_exprs {
            expr = EBinop(binop, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn additive_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((multiplicative_expr, many0(tuple((additive_op, multiplicative_expr)))))(input)?;

        for (binop, other_expr) in other_exprs {
            expr = EBinop(binop, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn relation_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((additive_expr, many0(tuple((relation_op, additive_expr)))))(input)?;

        for (binop, other_expr) in other_exprs {
            expr = EBinop(binop, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn eq_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((relation_expr, many0(tuple((eq_op, relation_expr)))))(input)?;

        for (binop, other_expr) in other_exprs {
            expr = EBinop(binop, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn and_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((eq_expr, many0(tuple((Token::AmpersandAmpersand, eq_expr)))))(input)?;

        for (_, other_expr) in other_exprs {
            expr = EBinop(Binop::BAnd, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn or_expr(input: Input) -> IResult<Input, Expr> {
        let (input, (mut expr, other_exprs)) =
            tuple((and_expr, many0(tuple((Token::VerticalBarVerticalBar, and_expr)))))(input)?;

        for (_, other_expr) in other_exprs {
            expr = EBinop(Binop::BOr, Box::new(expr), Box::new(other_expr))
        }

        Ok((input, expr))
    }

    fn cond_expr(input: Input) -> IResult<Input, Expr> {
        or_expr(input)
    }

    fn assign_expr(input: Input) -> IResult<Input, Expr> {
        alt((
            map(
                tuple((unary_expr, Token::Eq, unary_expr)),
                |(assignee, _, assigned)| Expr::EAssign(Box::new(assignee), Box::new(assigned)),
            ),
            cond_expr
        ))(input)
    }

    fn integer(input: Input) -> IResult<Input, Const> {
        match input.peek() {
            Some((Token::HexConstant(x), _)) => Ok((input.advance(), x)),
            Some((Token::DecimalConstant(x), _)) => Ok((input.advance(), x)),
            Some((Token::OctalConstant(x), _)) => Ok((input.advance(), x)),
            _ => Err(
                Error(
                    nom::error::Error::new(input, ErrorKind::IsA),
                ),
            )
        }
    }

    #[cfg(test)]
    pub mod tests {
        use logos_nom_bridge::Tokens;
        use crate::parser::raw_nom::parse_file;

        #[test]
        fn test_raw() {
            let input = "struct S { int a; };";
            let tokens = Tokens::new(input);

            let (rest, parsed) = parse_file(tokens).unwrap();

            println!("{:?}", parsed)
        }
    }
}

/*
pub mod raw {
    use pest::Parser;
    use crate::parser::Rule;

    pub fn parse(file: &str) -> Result<File, Error> {
        let mut parsed = MiniCParser::parse(Rule::file, file)?;

        Ok(parse_file(parsed.next().unwrap()))
    }

    fn parse_file(pair: Pair<Rule>) -> File {
        let mut funs = Vec::new();
        let mut structs = Vec::new();

        for pair in pair.into_inner() {
            let inner_pair = pair.into_inner().next().unwrap();
            match inner_pair.as_rule() {
                Rule::decl_struct => structs.push(parse_decl_struct(inner_pair)),
                Rule::decl_fun => funs.push(parse_decl_fun(inner_pair)),
                _ => unreachable!()
            }
        }

        File { funs, structs }
    }

    fn parse_decl_struct(pair: Pair<Rule>) -> Struct {
        let mut sub = pair.into_inner();
        let identifier = sub.next().unwrap();

        let mut fields = Vec::new();

        Struct { name: String::from(identifier.as_str()), fields }
    }

    fn parse_decl_fun(_pair: Pair<Rule>) -> Fun {
        Fun { typ: Typ::TInt, body: Block { stmts: Vec::new(), vars: Vec::new() }, ident: String::from(""), formals: Vec::new() }
    }


    pub type Error = pest::error::Error<Rule>;


}
*/
