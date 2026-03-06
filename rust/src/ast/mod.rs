//! AST-based JSE implementation.
//!
//! Following the design in docs/regular.md:
//! - AST nodes contain env (passed during construction)
//! - AST nodes have apply() method for execution
//! - Two-env pattern enables static scoping

pub mod nodes;
pub mod parser;

pub use nodes::AstNode;
pub use parser::Parser;

/// Error type for AST operations
#[derive(Debug, thiserror::Error)]
pub enum AstError {
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Arity error: {0}")]
    ArityError(String),

    #[error("Evaluation error: {0}")]
    EvalError(String),
}
