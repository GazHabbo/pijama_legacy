use std::fmt::{Display, Formatter, Result};

use pijama_ast::node::Block;
use pijama_common::{location::Located, BinOp, Literal, Local, Primitive, UnOp};
use pijama_ty::Ty;

pub use lower::{LowerError, LowerResult};

mod lower;

#[derive(Debug)]
pub enum LetKind {
    NonRec(Option<Located<Ty>>),
    Rec(Located<Ty>),
}

#[derive(Debug)]
pub enum Term<'a> {
    Var(Local<'a>),
    Abs(Local<'a>, Ty, Box<Located<Term<'a>>>),
    UnaryOp(UnOp, Box<Located<Term<'a>>>),
    BinaryOp(BinOp, Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    App(Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    Lit(Literal),
    Cond(
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
    ),
    Let(
        LetKind,
        Located<Local<'a>>,
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
    ),
    Seq(Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    PrimFn(Primitive),
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Term::Var(var) => write!(f, "{}", var),
            Term::Abs(name, ty, term) => write!(f, "(λ{}:{}. {})", name, ty, term),
            Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
            Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Let(LetKind::Rec(ty), name, t1, t2) => {
                write!(f, "(let rec {} : {} = {} in {})", name, ty.content, t1, t2)
            }
            Term::Let(LetKind::NonRec(Some(ty)), name, t1, t2) => {
                write!(f, "(let {} : {} = {} in {})", name, ty.content, t1, t2)
            }
            Term::Let(LetKind::NonRec(None), name, t1, t2) => {
                write!(f, "(let {} = {} in {})", name, t1, t2)
            }
            Term::Seq(t1, t2) => write!(f, "{} ; {}", t1, t2),
            Term::PrimFn(prim) => write!(f, "{}", prim),
        }
    }
}

impl<'a> Term<'a> {
    pub fn from_ast(blk: Block<'a>) -> LowerResult<Located<Self>> {
        lower::lower_block(blk)
    }
}
