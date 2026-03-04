use jse::{Engine, ExpressionEnv};
use serde_json::json;

fn engine() -> Engine<ExpressionEnv> {
    Engine::new(ExpressionEnv)
}

#[test]
fn and_basic() {
    assert_eq!(
        engine()
            .execute(&json!(["$and", true, true, true]))
            .unwrap(),
        json!(true)
    );
    assert_eq!(
        engine()
            .execute(&json!(["$and", true, false, true]))
            .unwrap(),
        json!(false)
    );
}

#[test]
fn or_basic() {
    assert_eq!(
        engine()
            .execute(&json!(["$or", false, false, true]))
            .unwrap(),
        json!(true)
    );
    assert_eq!(
        engine()
            .execute(&json!(["$or", false, false, false]))
            .unwrap(),
        json!(false)
    );
}

#[test]
fn not_basic() {
    assert_eq!(
        engine().execute(&json!(["$not", true])).unwrap(),
        json!(false)
    );
    assert_eq!(
        engine().execute(&json!(["$not", false])).unwrap(),
        json!(true)
    );
}

#[test]
fn nested_logic() {
    let expr = json!([
        "$or",
        ["$and", true, ["$not", false]],
        ["$and", false, true]
    ]);
    assert_eq!(engine().execute(&expr).unwrap(), json!(true));
}

#[test]
fn deep_nesting() {
    let expr = json!([
        "$not",
        [
            "$or",
            ["$and", false, ["$not", false]],
            ["$not", true]
        ]
    ]);
    assert_eq!(engine().execute(&expr).unwrap(), json!(true));
}
