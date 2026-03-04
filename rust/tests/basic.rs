use jse::{Engine, ExpressionEnv};
use serde_json::json;

fn engine() -> Engine<ExpressionEnv> {
    Engine::new(ExpressionEnv)
}

#[test]
fn number_expr() {
    let result = engine().execute(&json!(42)).unwrap();
    assert_eq!(result, json!(42));
}

#[test]
fn float_expr() {
    let result = engine().execute(&json!(3.14)).unwrap();
    assert_eq!(result, json!(3.14));
}

#[test]
fn string_expr() {
    let result = engine().execute(&json!("hello")).unwrap();
    assert_eq!(result, json!("hello"));
}

#[test]
fn boolean_expr() {
    assert_eq!(engine().execute(&json!(true)).unwrap(), json!(true));
    assert_eq!(engine().execute(&json!(false)).unwrap(), json!(false));
}

#[test]
fn null_expr() {
    assert!(engine().execute(&json!(null)).unwrap().is_null());
}

#[test]
fn array_expr() {
    let result = engine().execute(&json!([1, 2, 3])).unwrap();
    assert!(result.is_array());
    assert_eq!(result, json!([1, 2, 3]));
}

#[test]
fn object_expr() {
    let result = engine()
        .execute(&json!({"a": 1, "b": "x"}))
        .unwrap();
    assert!(result.is_object());
    assert_eq!(result, json!({"a": 1, "b": "x"}));
}
