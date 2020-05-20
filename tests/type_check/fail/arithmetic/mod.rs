use crate::{test_type, test_type_for_all_integer_binops};

use pijama::{
    ast::Located,
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_minus,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    }))
);

// Test all int binary operators with a bool and a int argument
test_type_for_all_integer_binops!(
    mixed_types_placeholder,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    })),
    OPERATOR
);

// Test all int binary operators with bool arguments
test_type_for_all_integer_binops!(
    wrong_type_placeholder,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    })),
    OPERATOR
);
