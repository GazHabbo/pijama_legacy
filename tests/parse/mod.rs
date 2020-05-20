use std::include_str;

use pijama::{
    ast::{self, BinOp::*, Node::*, UnOp},
    parser::parse,
    ty::{Binding, Ty},
    LangResult,
};

use crate::util::DummyLoc;

#[test]
fn name() -> LangResult<'static, ()> {
    let input = include_str!("name.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Name(ast::Name("x")).loc(),
        Name(ast::Name("foo")).loc(),
        Name(ast::Name("foo_bar")).loc(),
    ];

    assert_eq!(expected[0], result[0], "single letter");
    assert_eq!(expected[1], result[1], "word");
    assert_eq!(expected[2], result[2], "snake case");
    Ok(())
}

#[test]
fn literal() -> LangResult<'static, ()> {
    let input = include_str!("literal.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Literal(ast::Literal::Number(0)).loc(),
        Literal(ast::Literal::Number(-1)).loc(),
        Literal(ast::Literal::Number(14142)).loc(),
        Literal(ast::Literal::Bool(true)).loc(),
        Literal(ast::Literal::Bool(false)).loc(),
        Literal(ast::Literal::Unit).loc(),
    ];

    assert_eq!(expected[0], result[0], "integer");
    assert_eq!(expected[1], result[1], "negative integer");
    assert_eq!(expected[2], result[2], "large integer");
    assert_eq!(expected[3], result[3], "true");
    assert_eq!(expected[4], result[4], "false");
    assert_eq!(expected[5], result[5], "unit");
    Ok(())
}

#[test]
fn binary_op() -> LangResult<'static, ()> {
    let input = include_str!("bin_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Add,
            box Name(ast::Name("a")).loc(),
            box Name(ast::Name("b")).loc(),
        )
        .loc(),
        BinaryOp(
            Add,
            box BinaryOp(
                Add,
                box Name(ast::Name("a")).loc(),
                box Name(ast::Name("b")).loc(),
            )
            .loc(),
            box Name(ast::Name("c")).loc(),
        )
        .loc(),
        BinaryOp(
            Add,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                Add,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn unary_op() -> LangResult<'static, ()> {
    let input = include_str!("un_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        UnaryOp(UnOp::Neg, box Name(ast::Name("x")).loc()).loc(),
        UnaryOp(UnOp::Not, box Name(ast::Name("x")).loc()).loc(),
        UnaryOp(
            UnOp::Not,
            box UnaryOp(UnOp::Not, box Name(ast::Name("x")).loc()).loc(),
        )
        .loc(),
        UnaryOp(UnOp::Not, box Name(ast::Name("x")).loc()).loc(),
    ];

    assert_eq!(expected[0], result[0], "minus");
    assert_eq!(expected[1], result[1], "not");
    assert_eq!(expected[2], result[2], "double");
    assert_eq!(expected[3], result[3], "brackets");
    Ok(())
}

#[test]
fn logic_op() -> LangResult<'static, ()> {
    let input = include_str!("logic_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            And,
            box Name(ast::Name("a")).loc(),
            box Name(ast::Name("b")).loc(),
        )
        .loc(),
        BinaryOp(
            Or,
            box BinaryOp(
                And,
                box Name(ast::Name("a")).loc(),
                box Name(ast::Name("b")).loc(),
            )
            .loc(),
            box Name(ast::Name("c")).loc(),
        )
        .loc(),
        BinaryOp(
            And,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                Or,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn bit_op() -> LangResult<'static, ()> {
    let input = include_str!("bit_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            BitAnd,
            box Name(ast::Name("a")).loc(),
            box Name(ast::Name("b")).loc(),
        )
        .loc(),
        BinaryOp(
            BitXor,
            box BinaryOp(
                BitOr,
                box BinaryOp(
                    BitAnd,
                    box Name(ast::Name("a")).loc(),
                    box Name(ast::Name("b")).loc(),
                )
                .loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
            box Name(ast::Name("d")).loc(),
        )
        .loc(),
        BinaryOp(
            BitXor,
            box BinaryOp(
                BitAnd,
                box Name(ast::Name("a")).loc(),
                box BinaryOp(
                    BitOr,
                    box Name(ast::Name("b")).loc(),
                    box Name(ast::Name("c")).loc(),
                )
                .loc(),
            )
            .loc(),
            box Name(ast::Name("d")).loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn let_bind() -> LangResult<'static, ()> {
    let input = include_str!("let_bind.pj");
    let result = parse(input)?.content;
    let expected = vec![
        LetBind(ast::Name("x").loc(), None, box Name(ast::Name("y")).loc()).loc(),
        LetBind(
            ast::Name("x").loc(),
            None,
            box BinaryOp(
                Add,
                box Name(ast::Name("y")).loc(),
                box Name(ast::Name("z")).loc(),
            )
            .loc(),
        )
        .loc(),
        LetBind(
            ast::Name("x").loc(),
            Some(Ty::Int.loc()),
            box Name(ast::Name("y")).loc(),
        )
        .loc(),
        LetBind(
            ast::Name("foo").loc(),
            None,
            box FnDef(
                None,
                vec![Binding {
                    name: ast::Name("x"),
                    ty: Ty::Int,
                }
                .loc()],
                vec![Name(ast::Name("x")).loc()].loc(),
                None,
            )
            .loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "bind to bin op");
    assert_eq!(expected[2], result[2], "type binding");
    assert_eq!(expected[3], result[3], "bind to nameless function");
    Ok(())
}

#[test]
fn cond() -> LangResult<'static, ()> {
    let input = include_str!("cond.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Cond(
            vec![Name(ast::Name("x")).loc()].loc(),
            vec![Name(ast::Name("y")).loc()].loc(),
            vec![Name(ast::Name("z")).loc()].loc(),
        )
        .loc(),
        Cond(
            vec![Name(ast::Name("u")).loc(), Name(ast::Name("v")).loc()].loc(),
            vec![Name(ast::Name("w")).loc(), Name(ast::Name("x")).loc()].loc(),
            vec![Name(ast::Name("y")).loc(), Name(ast::Name("z")).loc()].loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple blocks");
    assert_eq!(expected[1], result[1], "long blocks");
    Ok(())
}

#[test]
fn call() -> LangResult<'static, ()> {
    let input = include_str!("call.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Call(ast::Name("x").loc(), vec![]).loc(),
        Call(ast::Name("x").loc(), vec![Name(ast::Name("y")).loc()]).loc(),
        Call(
            ast::Name("x").loc(),
            vec![Name(ast::Name("y")).loc(), Name(ast::Name("z")).loc()],
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "nullary call");
    assert_eq!(expected[1], result[1], "unary call");
    assert_eq!(expected[2], result[2], "binary call");
    Ok(())
}

#[test]
fn fn_def() -> LangResult<'static, ()> {
    let input = include_str!("fn_def.pj");
    let result = parse(input)?.content;
    let expected = vec![
        FnDef(Some(ast::Name("foo").loc()), vec![], vec![].loc(), None).loc(),
        FnDef(
            Some(ast::Name("foo").loc()),
            vec![Binding {
                name: ast::Name("x"),
                ty: Ty::Int,
            }
            .loc()],
            vec![Name(ast::Name("x")).loc()].loc(),
            None,
        )
        .loc(),
        FnRecDef(
            ast::Name("foo").loc(),
            vec![],
            vec![Call(ast::Name("foo").loc(), vec![]).loc()].loc(),
            Ty::Unit.loc(),
        )
        .loc(),
        FnDef(
            Some(ast::Name("foo").loc()),
            vec![
                Binding {
                    name: ast::Name("x"),
                    ty: Ty::Int,
                }
                .loc(),
                Binding {
                    name: ast::Name("y"),
                    ty: Ty::Int,
                }
                .loc(),
            ],
            vec![Name(ast::Name("x")).loc(), Name(ast::Name("y")).loc()].loc(),
            None,
        )
        .loc(),
        FnDef(
            None,
            vec![Binding {
                name: ast::Name("x"),
                ty: Ty::Int,
            }
            .loc()],
            vec![Name(ast::Name("x")).loc()].loc(),
            None,
        )
        .loc(),
        FnDef(
            None,
            vec![Binding {
                name: ast::Name("x"),
                ty: Ty::Int,
            }
            .loc()],
            vec![Name(ast::Name("x")).loc()].loc(),
            None,
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "nullary def");
    assert_eq!(expected[1], result[1], "unary def");
    assert_eq!(expected[2], result[2], "recursive def");
    assert_eq!(expected[3], result[3], "long body");
    assert_eq!(expected[4], result[4], "nameless");
    assert_eq!(expected[5], result[5], "nameless with space");
    Ok(())
}

#[test]
fn precedence() -> LangResult<'static, ()> {
    let input = include_str!("precedence.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Add,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                Mul,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
        BinaryOp(
            BitAnd,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                Add,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
        BinaryOp(
            Eq,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                BitAnd,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
        BinaryOp(
            And,
            box Name(ast::Name("a")).loc(),
            box BinaryOp(
                Eq,
                box Name(ast::Name("b")).loc(),
                box Name(ast::Name("c")).loc(),
            )
            .loc(),
        )
        .loc(),
    ];
    assert_eq!(expected[0], result[0], "mul precedes add");
    assert_eq!(expected[1], result[1], "add precedes bitwise and");
    assert_eq!(expected[2], result[2], "bitwise and precedes equal");
    assert_eq!(expected[3], result[3], "equal precedes and");
    Ok(())
}

#[test]
fn cmp_and_shift() -> LangResult<'static, ()> {
    let input = include_str!("cmp_and_shift.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Lt,
            box BinaryOp(
                Shl,
                box Name(ast::Name("a")).loc(),
                box Name(ast::Name("b")).loc(),
            )
            .loc(),
            box BinaryOp(
                Shl,
                box Name(ast::Name("c")).loc(),
                box Name(ast::Name("d")).loc(),
            )
            .loc(),
        )
        .loc(),
        BinaryOp(
            Gt,
            box BinaryOp(
                Shr,
                box Name(ast::Name("a")).loc(),
                box Name(ast::Name("b")).loc(),
            )
            .loc(),
            box BinaryOp(
                Shr,
                box Name(ast::Name("c")).loc(),
                box Name(ast::Name("d")).loc(),
            )
            .loc(),
        )
        .loc(),
        BinaryOp(
            Shr,
            box BinaryOp(
                Shr,
                box Name(ast::Name("a")).loc(),
                box BinaryOp(
                    Gt,
                    box Name(ast::Name("b")).loc(),
                    box Name(ast::Name("c")).loc(),
                )
                .loc(),
            )
            .loc(),
            box Name(ast::Name("d")).loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "left shift");
    assert_eq!(expected[1], result[1], "right shift");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}
