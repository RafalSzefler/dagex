use multi_valued_logic::tribool::TriBool;
use rstest::rstest;


#[rstest]
#[case(TriBool::TRUE, TriBool::TRUE)]
#[case(TriBool::UNKNOWN, TriBool::UNKNOWN)]
#[case(TriBool::FALSE, TriBool::FALSE)]
fn test_eq(#[case] left: TriBool, #[case] right: TriBool) {
    assert_eq!(left, right);
}

#[rstest]
#[case(TriBool::TRUE, TriBool::FALSE)]
#[case(TriBool::TRUE, TriBool::UNKNOWN)]
#[case(TriBool::UNKNOWN, TriBool::FALSE)]
#[case(TriBool::UNKNOWN, TriBool::TRUE)]
#[case(TriBool::FALSE, TriBool::UNKNOWN)]
#[case(TriBool::FALSE, TriBool::TRUE)]
fn test_neq(#[case] left: TriBool, #[case] right: TriBool) {
    assert_ne!(left, right);
}

#[rstest]
#[case(TriBool::TRUE, TriBool::TRUE, TriBool::TRUE)]
#[case(TriBool::TRUE, TriBool::FALSE, TriBool::FALSE)]
#[case(TriBool::TRUE, TriBool::UNKNOWN, TriBool::UNKNOWN)]
#[case(TriBool::FALSE, TriBool::FALSE, TriBool::FALSE)]
#[case(TriBool::FALSE, TriBool::TRUE, TriBool::FALSE)]
#[case(TriBool::FALSE, TriBool::UNKNOWN, TriBool::FALSE)]
#[case(TriBool::UNKNOWN, TriBool::TRUE, TriBool::UNKNOWN)]
#[case(TriBool::UNKNOWN, TriBool::FALSE, TriBool::FALSE)]
#[case(TriBool::UNKNOWN, TriBool::UNKNOWN, TriBool::UNKNOWN)]
fn test_and(#[case] left: TriBool, #[case] right: TriBool, #[case] expected: TriBool) {
    assert_eq!(left.and(right), expected);
}

#[rstest]
#[case(TriBool::TRUE, TriBool::TRUE, TriBool::TRUE)]
#[case(TriBool::TRUE, TriBool::FALSE, TriBool::TRUE)]
#[case(TriBool::TRUE, TriBool::UNKNOWN, TriBool::TRUE)]
#[case(TriBool::FALSE, TriBool::FALSE, TriBool::FALSE)]
#[case(TriBool::FALSE, TriBool::TRUE, TriBool::TRUE)]
#[case(TriBool::FALSE, TriBool::UNKNOWN, TriBool::UNKNOWN)]
#[case(TriBool::UNKNOWN, TriBool::TRUE, TriBool::TRUE)]
#[case(TriBool::UNKNOWN, TriBool::FALSE, TriBool::UNKNOWN)]
#[case(TriBool::UNKNOWN, TriBool::UNKNOWN, TriBool::UNKNOWN)]
fn test_or(#[case] left: TriBool, #[case] right: TriBool, #[case] expected: TriBool) {
    assert_eq!(left.or(right), expected);
}

#[rstest]
#[case(TriBool::TRUE, true)]
#[case(TriBool::FALSE, false)]
#[case(TriBool::UNKNOWN, false)]
fn test_is_certain(#[case] tri: TriBool, #[case] expected: bool) {
    assert_eq!(tri.is_certain(), expected);
}

#[rstest]
#[case(TriBool::TRUE, true)]
#[case(TriBool::FALSE, false)]
#[case(TriBool::UNKNOWN, true)]
fn test_is_possible(#[case] tri: TriBool, #[case] expected: bool) {
    assert_eq!(tri.is_possible(), expected);
}

#[rstest]
#[case(TriBool::TRUE, TriBool::FALSE)]
#[case(TriBool::UNKNOWN, TriBool::UNKNOWN)]
#[case(TriBool::FALSE, TriBool::TRUE)]
fn test_neg(#[case] tri: TriBool, #[case] expected: TriBool) {
    assert_eq!(tri.neg(), expected);
}
