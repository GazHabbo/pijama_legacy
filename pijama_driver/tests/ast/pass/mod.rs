use crate::test_type;

use pijama_ty::Ty;

test_type!(
    contained_shadowing_allows_recursion,
    Ok(&Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
test_type!(
    shadowing_is_not_recursion,
    Ok(&Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
test_type!(
    shadowing_is_not_recursion_2,
    Ok(&Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
test_type!(
    binding_persists_whole_block,
    Ok(&Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
