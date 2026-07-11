use insta::assert_debug_snapshot;

mod common;

use common::run;

// https://github.com/Paligo/xee/issues/5
//
// XPath 3.1 section 3.8.1 allows and/or to avoid evaluating an operand
// once the other operand determines the result.

#[test]
fn test_and_short_circuits_on_false() {
    assert_debug_snapshot!(run("false() and error()"));
}

#[test]
fn test_or_short_circuits_on_true() {
    assert_debug_snapshot!(run("true() or error()"));
}

#[test]
fn test_and_still_raises_from_right_operand() {
    assert_debug_snapshot!(run("true() and error()"));
}

#[test]
fn test_or_still_raises_from_right_operand() {
    assert_debug_snapshot!(run("false() or error()"));
}

#[test]
fn test_and_takes_effective_boolean_values() {
    assert_debug_snapshot!(run(r#"1 and "x""#));
}

#[test]
fn test_or_takes_effective_boolean_values() {
    assert_debug_snapshot!(run(r#"0 or """#));
}

#[test]
fn test_and_rejects_sequences_without_effective_boolean_value() {
    assert_debug_snapshot!(run("(1, 2) and true()"));
}

#[test]
fn test_or_right_operand_effective_boolean_value() {
    // the right operand's value is reduced to its effective boolean
    // value, not returned as-is
    assert_debug_snapshot!(run(r#"false() or "x""#));
}

#[test]
fn test_and_raises_forg0006_from_right_operand() {
    // the right operand is still evaluated when the left is true, so its
    // lack of an effective boolean value is an error
    assert_debug_snapshot!(run("true() and (1, 2)"));
}

#[test]
fn test_three_term_and_chain_short_circuits() {
    assert_debug_snapshot!(run("false() and error() and error()"));
}

#[test]
fn test_three_term_or_chain_short_circuits() {
    assert_debug_snapshot!(run("true() or error() or error()"));
}

#[test]
fn test_mixed_precedence_short_circuits() {
    // parses as true() or (false() and error()); the outer or is true,
    // so neither the inner and nor error() is evaluated
    assert_debug_snapshot!(run("true() or false() and error()"));
}
