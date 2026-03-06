//! Utility functors for JSE.
//!
//! Following Python's functors/utils.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::env::{Env, Functor};
use crate::ast::AstError;

pub fn not(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    let value = args.first().map_or(true, |v| !is_truthy(v));
    Ok(Value::Bool(value))
}

pub fn listp(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    let result = args.first().map_or(false, |v| matches!(v, Value::Array(_)));
    Ok(Value::Bool(result))
}

pub fn mapp(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    let result = args.first().map_or(false, |v| matches!(v, Value::Object(_)));
    Ok(Value::Bool(result))
}

pub fn nullp(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    let result = args.first().map_or(true, |v| matches!(v, Value::Null));
    Ok(Value::Bool(result))
}

pub fn get(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 2 {
        return Err(AstError::ArityError("$get requires (collection, key) arguments".to_string()));
    }

    match &args[0] {
        Value::Object(obj) => {
            let key = args[1].as_str()
                .ok_or_else(|| AstError::TypeError("$get on object requires string key".to_string()))?;
            Ok(obj.get(key).cloned().unwrap_or(Value::Null))
        }
        Value::Array(arr) => {
            let index = args[1].as_i64()
                .ok_or_else(|| AstError::TypeError("$get on array requires integer index".to_string()))?
                as usize;
            if index < arr.len() {
                Ok(arr[index].clone())
            } else {
                Err(AstError::EvalError(format!("Index {} out of range", index)))
            }
        }
        _ => Err(AstError::TypeError("$get first argument must be object or array".to_string())),
    }
}

pub fn set(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 3 {
        return Err(AstError::ArityError("$set requires (collection, key, value) arguments".to_string()));
    }

    // For immutable values, return a modified copy
    match &args[0] {
        Value::Object(obj) => {
            let key = args[1].as_str()
                .ok_or_else(|| AstError::TypeError("$set on object requires string key".to_string()))?;
            let mut new_obj = obj.clone();
            new_obj.insert(key.to_string(), args[2].clone());
            Ok(Value::Object(new_obj))
        }
        Value::Array(arr) => {
            let index = args[1].as_i64()
                .ok_or_else(|| AstError::TypeError("$set on array requires integer index".to_string()))?
                as usize;
            if index >= arr.len() {
                return Err(AstError::EvalError(format!("Index {} out of range", index)));
            }
            let mut new_arr = arr.clone();
            new_arr[index] = args[2].clone();
            Ok(Value::Array(new_arr))
        }
        _ => Err(AstError::TypeError("$set first argument must be object or array".to_string())),
    }
}

pub fn del(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 2 {
        return Err(AstError::ArityError("$del requires (collection, key) arguments".to_string()));
    }

    match &args[0] {
        Value::Object(obj) => {
            let key = args[1].as_str()
                .ok_or_else(|| AstError::TypeError("$del on object requires string key".to_string()))?;
            let mut new_obj = obj.clone();
            new_obj.remove(key);
            Ok(Value::Object(new_obj))
        }
        Value::Array(arr) => {
            let index = args[1].as_i64()
                .ok_or_else(|| AstError::TypeError("$del on array requires integer index".to_string()))?
                as usize;
            if index >= arr.len() {
                return Err(AstError::EvalError(format!("Index {} out of range", index)));
            }
            let mut new_arr = arr.clone();
            new_arr.remove(index);
            Ok(Value::Array(new_arr))
        }
        _ => Err(AstError::TypeError("$del first argument must be object or array".to_string())),
    }
}

pub fn conj(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() != 2 {
        return Err(AstError::ArityError("$conj requires exactly 2 arguments".to_string()));
    }

    match &args[1] {
        Value::Array(arr) => {
            let mut result = arr.clone();
            result.push(args[0].clone());
            Ok(Value::Array(result))
        }
        _ => Err(AstError::TypeError("$conj second argument must be a list".to_string())),
    }
}

pub fn and(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    Ok(Value::Bool(args.iter().all(is_truthy)))
}

pub fn or(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    Ok(Value::Bool(args.iter().any(is_truthy)))
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Null => false,
        _ => true,
    }
}

pub fn utils_functors() -> std::collections::HashMap<&'static str, Functor> {
    let mut m = std::collections::HashMap::new();
    m.insert("$not", not as Functor);
    m.insert("$list?", listp as Functor);
    m.insert("$map?", mapp as Functor);
    m.insert("$null?", nullp as Functor);
    m.insert("$get", get as Functor);
    m.insert("$set", set as Functor);
    m.insert("$del", del as Functor);
    m.insert("$conj", conj as Functor);
    m.insert("$and", and as Functor);
    m.insert("$or", or as Functor);
    m
}
