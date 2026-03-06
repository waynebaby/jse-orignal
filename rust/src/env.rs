//! Environment for JSE execution with scope chaining.
//!
//! Following Python's env.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use serde_json::Value;
use crate::ast::{AstNode, AstError};

/// Functor type: function that takes env and arguments, returns Value
pub type Functor = fn(&Rc<RefCell<Env>>, &[Value]) -> Result<Value, AstError>;

/// Environment for JSE execution with scope chaining
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    bindings: HashMap<String, Value>,
    functors: HashMap<String, Functor>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            bindings: HashMap::new(),
            functors: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            parent,
            bindings: HashMap::new(),
            functors: HashMap::new(),
        }
    }

    /// Resolve symbol to value by searching up the scope chain
    pub fn resolve(&self, symbol: &str) -> Option<Value> {
        if let Some(value) = self.bindings.get(symbol) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().resolve(symbol)
        } else {
            None
        }
    }

    /// Register a new symbol binding (throws if exists)
    pub fn register(&mut self, name: String, value: Value) -> Result<(), AstError> {
        if self.bindings.contains_key(&name) {
            return Err(AstError::EvalError(
                format!("Symbol '{}' already exists in current scope", name)
            ));
        }
        self.bindings.insert(name, value);
        Ok(())
    }

    /// Set a symbol binding (overwrites if exists)
    pub fn set(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    /// Check if symbol exists in scope chain
    pub fn exists(&self, name: &str) -> bool {
        self.bindings.contains_key(name) ||
        self.parent.as_ref().map_or(false, |p| p.borrow().exists(name))
    }

    /// Register a functor
    pub fn register_functor(&mut self, name: String, functor: Functor) {
        self.functors.insert(name, functor);
    }

    /// Load a module of functors
    pub fn load(&mut self, module: &HashMap<String, Functor>) {
        for (name, functor) in module {
            self.functors.insert(name.clone(), *functor);
        }
    }

    /// Apply a functor with AST nodes (for special forms)
    pub fn apply_functor(
        &self,
        name: &str,
        env: &Rc<RefCell<Env>>,
        args: &[&dyn AstNode]
    ) -> Result<Value, AstError> {
        let functor = self.functors.get(name)
            .ok_or_else(|| AstError::SymbolNotFound(name.to_string()))?;

        // Convert AST nodes to their JSON representation for special forms
        let json_args: Result<Vec<Value>, _> = args.iter()
            .map(|node| Ok(node.to_json()))
            .collect();

        functor(env, &json_args?)
    }

    /// Apply a functor with evaluated values
    pub fn apply_functor_with_values(
        &self,
        name: &str,
        env: &Rc<RefCell<Env>>,
        args: &[Value]
    ) -> Result<Value, AstError> {
        let functor = self.resolve_functor(name)
            .ok_or_else(|| AstError::SymbolNotFound(name.to_string()))?;

        functor(env, args)
    }

    /// Resolve a functor from the environment chain
    pub fn resolve_functor(&self, name: &str) -> Option<Functor> {
        if let Some(&functor) = self.functors.get(name) {
            Some(functor)
        } else if let Some(parent) = &self.parent {
            parent.borrow().resolve_functor(name)
        } else {
            None
        }
    }
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.as_ref().map(Rc::clone),
            bindings: self.bindings.clone(),
            functors: self.functors.clone(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

/// Expression-only environment for basic evaluation
#[derive(Default)]
pub struct ExpressionEnv {
    inner: Rc<RefCell<Env>>,
}

impl ExpressionEnv {
    pub fn new() -> Self {
        let mut inner = Env::new();
        // Load default functors - convert to String keys
        for (name, functor) in crate::functors::builtin::builtin_functors() {
            inner.register_functor(name.to_string(), functor);
        }
        for (name, functor) in crate::functors::lisp::lisp_functors() {
            inner.register_functor(name.to_string(), functor);
        }
        for (name, functor) in crate::functors::utils::utils_functors() {
            inner.register_functor(name.to_string(), functor);
        }
        for (name, functor) in crate::functors::sql::sql_functors() {
            inner.register_functor(name.to_string(), functor);
        }
        Self { inner: Rc::new(RefCell::new(inner)) }
    }

    pub fn as_env_ref(&self) -> &Rc<RefCell<Env>> {
        &self.inner
    }
}

// Keep the old trait for backward compatibility
/// Base environment for JSE execution.
/// This trait is deprecated; use the `Env` struct instead.
#[deprecated(note = "Use the Env struct instead")]
pub trait EnvTrait {
    /// Resolve a symbol to a value. Return `None` if not bound.
    fn resolve(&self, symbol: &str) -> Option<Value> {
        None
    }
}

// Implement the deprecated trait for Env
#[allow(deprecated)]
impl EnvTrait for Env {
    fn resolve(&self, symbol: &str) -> Option<Value> {
        self.resolve(symbol)
    }
}

#[allow(deprecated)]
impl EnvTrait for ExpressionEnv {
    fn resolve(&self, symbol: &str) -> Option<Value> {
        self.inner.borrow().resolve(symbol)
    }
}
