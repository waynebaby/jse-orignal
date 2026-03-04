use serde_json::Value;

/// Base environment for JSE execution.
/// Implement this trait to provide custom symbol resolution.
pub trait Env {
    /// Resolve a symbol to a value. Return `None` if not bound.
    fn resolve(&self, _symbol: &str) -> Option<Value> {
        None
    }
}

/// Expression-only environment for basic and logic evaluation.
#[derive(Default)]
pub struct ExpressionEnv;

impl Env for ExpressionEnv {}
