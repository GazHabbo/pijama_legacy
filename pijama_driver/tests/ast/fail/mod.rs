use crate::{test_type, util::dummy_loc};

use pijama_driver::LangError;
use pijama_hir::LowerError;

test_type!(
    detect_indirect_recursion,
    Err(LangError::Lower(LowerError::RequiredTy(dummy_loc())))
);

test_type!(
    detect_recursion_after_shadowing,
    Err(LangError::Lower(LowerError::RequiredTy(dummy_loc())))
);

test_type!(
    detect_recursion_after_shadowing_2,
    Err(LangError::Lower(LowerError::RequiredTy(dummy_loc())))
);

test_type!(
    detect_recursion_inside_functions,
    Err(LangError::Lower(LowerError::RequiredTy(dummy_loc())))
);
