//! JSE execution engine with AST-based architecture.
//!
//! Following Python's engine.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::ast::{Parser, AstError};
use crate::env::{Env, ExpressionEnv, EnvTrait};

/// JSE execution error.
#[derive(Debug, thiserror::Error)]
pub enum JseError {
    #[error("AST error: {0}")]
    AstError(#[from] AstError),

    #[error("{0}")]
    Message(String),
}

impl JseError {
    fn msg(s: impl Into<String>) -> Self {
        JseError::Message(s.into())
    }
}

/// JSE execution engine with AST-based execution.
pub struct Engine<E> {
    env: E,
    parser: Parser,
}

impl<E: AsEnv> Engine<E> {
    pub fn new(env: E) -> Self {
        let env_ref = env.as_env_ref().clone();
        let parser = Parser::new(env_ref);
        Self { env, parser }
    }

    /// Execute a JSE expression.
    pub fn execute(&self, expr: &Value) -> Result<Value, JseError> {
        // Parse into AST
        let ast = self.parser.parse(expr)?;

        // Evaluate using environment
        let env_ref = self.env.as_env_ref();
        ast.apply(env_ref).map_err(JseError::from)
    }
}

// Backward compatibility with old Env trait
impl<E: EnvTrait> Engine<E> {
    pub fn with_env(env: E) -> Self {
        // Create a new ExpressionEnv wrapper for backward compatibility
        let expr_env = ExpressionEnv::new();
        Self {
            env,
            parser: Parser::new(expr_env.as_env_ref().clone()),
        }
    }
}

/// Trait for types that can provide an environment reference
pub trait AsEnv {
    fn as_env_ref(&self) -> &Rc<RefCell<Env>>;
}

impl AsEnv for ExpressionEnv {
    fn as_env_ref(&self) -> &Rc<RefCell<Env>> {
        // Use the public method from ExpressionEnv
        ExpressionEnv::as_env_ref(self)
    }
}

impl AsEnv for Rc<RefCell<Env>> {
    fn as_env_ref(&self) -> &Rc<RefCell<Env>> {
        self
    }
}
