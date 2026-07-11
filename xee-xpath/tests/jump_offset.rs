// Forward jumps over a large branch body must not overflow the
// displacement. Before jump offsets were widened to i32, a branch body
// over ~32KB of bytecode wrapped the offset and panicked at runtime.

use xee_xpath::{error, Documents, Queries, Query};

fn eval(xpath: &str) -> error::Result<String> {
    let mut documents = Documents::new();
    let queries = Queries::default();
    let q = queries.one(xpath, |_, item| Ok(item.try_into_value::<bool>()?))?;
    q.execute_build_context(&mut documents, |_| {})
        .map(|b| b.to_string())
}

// a flat expression whose bytecode is well over 32KB but is not deeply
// nested (so it does not hit the recursive-compilation depth limit)
fn large_branch_body() -> String {
    let members = vec!["1"; 13000].join(",");
    format!("(count([{members}]) = 999)")
}

#[test]
fn test_if_over_large_branch_body() {
    let body = large_branch_body();
    assert_eq!(
        eval(&format!("if (false()) then {body} else false()")).unwrap(),
        "false"
    );
    assert_eq!(
        eval(&format!("if (true()) then true() else {body}")).unwrap(),
        "true"
    );
}
