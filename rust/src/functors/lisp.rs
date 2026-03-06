//! LISP-enhanced functors for JSE.
//!
//! Following Python's functors/lisp.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::env::{Env, Functor};
use crate::ast::{AstError, Parser};

pub fn apply_fn(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 2 {
        return Err(AstError::ArityError("$apply requires (functor, arglist) arguments".to_string()));
    }

    // First arg should be a functor name
    let functor_name = args[0].as_str()
        .ok_or_else(|| AstError::TypeError("$apply first argument must be a functor name".to_string()))?;

    let arglist = &args[1];
    let arr = arglist.as_array()
        .ok_or_else(|| AstError::TypeError("$apply second argument must be a list".to_string()))?;

    env.borrow().apply_functor_with_values(functor_name, env, arr)
}

pub fn eval_expr(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.is_empty() {
        return Ok(Value::Null);
    }

    // Re-parse the argument and evaluate
    let parser = Parser::new(Rc::clone(env));
    let ast = parser.parse(&args[0])?;
    ast.apply(env)
}

pub fn lambda(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 2 {
        return Err(AstError::ArityError("$lambda requires (params, body) arguments".to_string()));
    }

    let params = args[0].as_array()
        .ok_or_else(|| AstError::TypeError("$lambda first argument must be a parameter list".to_string()))?;

    let param_names: Result<Vec<String>, _> = params.iter()
        .map(|p| {
            p.as_str()
                .filter(|s| s.starts_with('$'))
                .map(|s| s.to_string())
                .ok_or_else(|| AstError::TypeError("Lambda parameters must be symbols starting with $".to_string()))
        })
        .collect();

    let param_names = param_names?;
    let body = args[1].clone();

    // Store lambda as a special object that can be called
    // For now, return a placeholder
    let mut lambda_obj = serde_json::Map::new();
    lambda_obj.insert("__lambda__".to_string(), Value::Bool(true));
    lambda_obj.insert("params".to_string(), Value::Array(
        param_names.into_iter().map(Value::String).collect()
    ));
    lambda_obj.insert("body".to_string(), body);

    Ok(Value::Object(lambda_obj))
}

pub fn def(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() != 2 {
        return Err(AstError::ArityError("$def requires (name, value) arguments".to_string()));
    }

    let name = args[0].as_str()
        .filter(|s| s.starts_with('$'))
        .ok_or_else(|| AstError::TypeError("$def first argument must be a symbol".to_string()))?
        .to_string();

    let value = args[1].clone();
    env.borrow_mut().set(name.clone(), value.clone());
    Ok(value)
}

pub fn defn(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 3 {
        return Err(AstError::ArityError("$defn requires (name, params, body) arguments".to_string()));
    }

    let name = args[0].as_str()
        .filter(|s| s.starts_with('$'))
        .ok_or_else(|| AstError::TypeError("$defn first argument must be a symbol".to_string()))?
        .to_string();

    // Create lambda and register it
    let params = &args[1];
    let body = &args[2];

    let lambda_args = vec![params.clone(), body.clone()];
    let lambda_val = lambda(env, &lambda_args)?;

    env.borrow_mut().set(name.clone(), lambda_val);
    Ok(Value::String(name))
}

pub fn lisp_functors() -> std::collections::HashMap<&'static str, Functor> {
    let mut m = std::collections::HashMap::new();
    m.insert("$apply", apply_fn as Functor);
    m.insert("$eval", eval_expr as Functor);
    m.insert("$lambda", lambda as Functor);
    m.insert("$def", def as Functor);
    m.insert("$defn", defn as Functor);
    m
}
