//! Basic builtin functors for JSE.
//!
//! Following Python's functors/builtin.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::env::{Env, Functor};
use crate::ast::AstError;

pub fn quote(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    Ok(args.first().cloned().unwrap_or(Value::Null))
}

pub fn eq(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 2 {
        return Ok(Value::Bool(true));
    }
    if args.len() == 2 {
        return Ok(Value::Bool(args[0] == args[1]));
    }
    // Check all adjacent pairs are equal
    for i in 0..args.len() - 1 {
        if args[i] != args[i + 1] {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

pub fn cond(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Value::Array(clause) => {
                if clause.is_empty() {
                    continue;
                }

                // If this is the last clause and has only one element, it's the else clause
                if i == args.len() - 1 && clause.len() == 1 {
                    return Ok(clause[0].clone());
                }

                if clause.len() < 2 {
                    return Err(AstError::EvalError("$cond clauses must have at least 2 elements except the last".to_string()));
                }

                if is_truthy(&clause[0]) {
                    return Ok(clause[1].clone());
                }
            }
            _ => {
                return Err(AstError::EvalError("$cond clauses must be arrays".to_string()));
            }
        }
    }

    Ok(Value::Null)
}

pub fn head(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.is_empty() {
        return Err(AstError::ArityError("$head requires a list argument".to_string()));
    }

    match &args[0] {
        Value::Array(arr) if !arr.is_empty() => Ok(arr[0].clone()),
        Value::Array(_) => Err(AstError::EvalError("$head requires non-empty list".to_string())),
        _ => Err(AstError::TypeError("$head requires a list argument".to_string())),
    }
}

pub fn tail(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.is_empty() {
        return Err(AstError::ArityError("$tail requires a list argument".to_string()));
    }

    match &args[0] {
        Value::Array(arr) if arr.len() > 1 => Ok(Value::Array(arr[1..].to_vec())),
        Value::Array(_) => Ok(Value::Array(vec![])),
        _ => Err(AstError::TypeError("$tail requires a list argument".to_string())),
    }
}

pub fn atomp(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }

    let is_atom = matches!(args[0], Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_));
    Ok(Value::Bool(is_atom))
}

pub fn cons(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() != 2 {
        return Err(AstError::ArityError("$cons requires exactly 2 arguments".to_string()));
    }

    match &args[1] {
        Value::Array(arr) => {
            let mut result = vec![args[0].clone()];
            result.extend(arr.clone());
            Ok(Value::Array(result))
        }
        _ => Err(AstError::TypeError("$cons second argument must be a list".to_string())),
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Null => false,
        _ => true,
    }
}

pub fn builtin_functors() -> std::collections::HashMap<&'static str, Functor> {
    let mut m = std::collections::HashMap::new();
    m.insert("$quote", quote as Functor);
    m.insert("$eq", eq as Functor);
    m.insert("$cond", cond as Functor);
    m.insert("$head", head as Functor);
    m.insert("$tail", tail as Functor);
    m.insert("$atom?", atomp as Functor);
    m.insert("$cons", cons as Functor);
    m
}
