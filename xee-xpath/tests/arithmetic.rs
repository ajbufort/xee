use insta::assert_debug_snapshot;

mod common;

use common::run;

#[test]
fn test_add_int_to_double() {
    assert_debug_snapshot!(run("12 + 15.4e0"));
}

#[test]
fn test_add_int_to_decimal() {
    assert_debug_snapshot!(run("12 + 15.4"));
}

#[test]
fn test_mul_int() {
    assert_debug_snapshot!(run("12 * 15"));
}

#[test]
fn test_div_decimal() {
    assert_debug_snapshot!(run("12 div 3.0"));
}

#[test]
fn test_div_double() {
    assert_debug_snapshot!(run("12 div 3.0e0"));
}

#[test]
fn test_div_both_integers() {
    // return type is decimal
    assert_debug_snapshot!(run("12 div 3"));
}

#[test]
fn test_integer_div() {
    assert_debug_snapshot!(run("12 idiv 5"));
}

#[test]
fn test_mod() {
    assert_debug_snapshot!(run("12 mod 5"));
}

// https://www.w3.org/TR/xpath-31/#id-arithmetic rule 4: an untypedAtomic
// operand is cast to xs:double (binary ops already did this; these cover
// the unary operators)

#[test]
fn test_unary_minus_untyped_atomic() {
    assert_debug_snapshot!(run(r#"-(xs:untypedAtomic("3"))"#));
}

#[test]
fn test_unary_plus_untyped_atomic() {
    assert_debug_snapshot!(run(r#"+(xs:untypedAtomic("3.5"))"#));
}

#[test]
fn test_unary_minus_untyped_atomic_invalid() {
    assert_debug_snapshot!(run(r#"-(xs:untypedAtomic("abc"))"#));
}

#[test]
fn test_unary_minus_string_is_still_a_type_error() {
    assert_debug_snapshot!(run(r#"-("a")"#));
}

#[test]
fn test_unary_minus_untyped_atomic_with_whitespace() {
    // the rule-4 cast behaves like a real cast to xs:double, including
    // the whiteSpace collapse facet
    assert_debug_snapshot!(run(r#"-(xs:untypedAtomic("  3  "))"#));
}

#[test]
fn test_binary_add_untyped_atomic_with_whitespace() {
    assert_debug_snapshot!(run(r#"xs:untypedAtomic("  4  ") + 1"#));
}
