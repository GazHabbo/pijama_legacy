use thiserror::Error;

use std::io::Write;

use pijama_common::location::LocatedError;
use pijama_hir::LowerErrorKind;
use pijama_parser::{parse, ParsingErrorKind};
use pijama_tycheck::{ty_check, TyErrorKind};

use pijama_lir::Term as LirTerm;

use pijama_machine::{
    arithmetic::{Arithmetic, CheckedArithmetic, OverflowArithmetic},
    Machine, MachineBuilder,
};

pub type LangResult<T> = Result<T, LangError>;

pub type LangError = LocatedError<LangErrorKind>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LangErrorKind {
    #[error("{0}")]
    Ty(#[from] TyErrorKind),
    #[error("{0}")]
    Parse(#[from] ParsingErrorKind),
    #[error("{0}")]
    Lower(#[from] LowerErrorKind),
}

pub fn run_with_machine<W: Write, A: Arithmetic>(
    input: &str,
    mut machine: Machine<W, A>,
) -> LangResult<()> {
    let ast = parse(input).map_err(LocatedError::kind_into)?;
    let (hir, ctx) = pijama_hir::lower_ast(ast).map_err(LocatedError::kind_into)?;
    let (_ty, mut ctx) = ty_check(&hir, ctx).map_err(LocatedError::kind_into)?;
    let _mir = pijama_mir::Term::from_hir(&hir, &mut ctx);
    let lir = LirTerm::from_hir(hir);
    let _res = machine.evaluate(lir);
    Ok(())
}

pub fn run(input: &str, overflow_check: bool) -> LangResult<()> {
    if overflow_check {
        let machine = MachineBuilder::default()
            .with_arithmetic(CheckedArithmetic)
            .build();
        run_with_machine(input, machine)
    } else {
        let machine = MachineBuilder::default()
            .with_arithmetic(OverflowArithmetic)
            .build();
        run_with_machine(input, machine)
    }
}
