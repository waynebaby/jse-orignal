use jse::{Engine, Env, QUERY_FIELDS, builtin_functors, utils_functors, sql_functors};
use serde_json::json;
use std::rc::Rc;
use std::cell::RefCell;

fn engine() -> Engine {
    let env = Rc::new(RefCell::new(Env::new()));
    env.borrow_mut().load(&builtin_functors());
    env.borrow_mut().load(&utils_functors());
    env.borrow_mut().load(&sql_functors());
    Engine::new(env)
}

#[test]
fn basic_query() {
    let query = json!({
        "$expr": ["$pattern", "*", "author of", "*"]
    });
    let result = engine().execute(&query).unwrap();
    let sql = result.as_str().unwrap();
    assert!(sql.contains("select"));
    assert!(sql.contains("subject, predicate, object, meta"));
    assert!(sql.contains("from statement as s"));
    assert!(sql.contains("author of"));
    assert!(sql.contains("triple"));
    assert!(sql.contains("offset 0"));
    assert!(sql.contains("limit 100"));
}

#[test]
fn combined_query() {
    let query = json!({
        "$query": [
            "$and",
            [
                ["$pattern", "Liu Xin", "author of", "*"],
                ["$pattern", "*", "author of", "*"]
            ]
        ]
    });
    let result = engine().execute(&query).unwrap();
    let sql = result.as_str().unwrap();
    assert!(sql.contains(&format!("select {}", QUERY_FIELDS)));
    assert!(sql.contains("from statement"));
    assert!(sql.contains("Liu Xin"));
    assert!(sql.contains("author of"));
    assert!(sql.contains(" and "));
    assert!(sql.contains("offset 0"));
    assert!(sql.contains("limit 100"));
}
