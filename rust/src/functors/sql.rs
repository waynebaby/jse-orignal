//! SQL extension functors for JSE.
//!
//! Following Python's functors/sql.py pattern:

use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::env::{Env, Functor};
use crate::ast::AstError;
use crate::sql::{pattern_to_triple, triple_to_sql_condition};

pub const QUERY_FIELDS: &str = "subject, predicate, object, meta";

pub fn pattern(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 3 {
        return Err(AstError::ArityError("$pattern requires (subject, predicate, object)".to_string()));
    }

    let subj = args[0].as_str()
        .ok_or_else(|| AstError::TypeError("$pattern requires string arguments".to_string()))?;
    let pred = args[1].as_str()
        .ok_or_else(|| AstError::TypeError("$pattern requires string arguments".to_string()))?;
    let obj = args[2].as_str()
        .ok_or_else(|| AstError::TypeError("$pattern requires string arguments".to_string()))?;

    let triple = pattern_to_triple(subj, pred, obj);
    let cond = triple_to_sql_condition(&triple);

    let sql = format!(
        "select \n    subject, predicate, object, meta \nfrom statement as s \nwhere {} \noffset 0\nlimit 100 \n",
        cond
    );

    Ok(Value::String(sql))
}

pub fn expr(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.is_empty() {
        return Ok(Value::Null);
    }
    Ok(args[0].clone())
}

pub fn query(env: &Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, AstError> {
    if args.len() < 1 {
        return Err(AstError::ArityError("$query expects [op, patterns array] or [patterns array]".to_string()));
    }

    // Helper function to evaluate an expression
    fn eval_expr(env: &Rc<RefCell<Env>>, expr: &Value) -> Result<Value, AstError> {
        // Re-parse and evaluate the expression
        let parser = crate::ast::Parser::new(Rc::clone(env));
        let ast = parser.parse(expr)?;
        ast.apply(env)
    }

    // Extract patterns from raw JSON structure
    // Don't evaluate the top-level array yet
    let raw_value = &args[0];

    // Check if this is [op, patterns] format or just [patterns]
    let (patterns_array, has_op) = match raw_value {
        Value::Array(arr) if !arr.is_empty() => {
            // Check if first element is an operator
            if arr[0].is_string() {
                let s = arr[0].as_str().unwrap();
                if s.starts_with('$') && s != "$*" && !s.starts_with("$$") && s != "$expr" && s != "$query" && s != "$pattern" {
                    // This is [op, patterns] format
                    // Check if it's a logical operator that needs special handling
                    if s == "$and" || s == "$or" {
                        // For $and/$or with patterns, the patterns are in the second element
                        if let Some(patterns) = arr.get(1) {
                            (patterns, true)
                        } else {
                            (raw_value, false)
                        }
                    } else {
                        // Unknown operator - treat whole thing as patterns
                        (raw_value, false)
                    }
                } else {
                    // First element is not an operator - this is [patterns]
                    (raw_value, false)
                }
            } else {
                // First element is not a string - this is [patterns]
                (raw_value, false)
            }
        }
        _ => (raw_value, false),
    };

    let patterns = patterns_array.as_array()
        .ok_or_else(|| AstError::TypeError("$query argument must be an array".to_string()))?;

    let mut conditions = Vec::new();
    for pattern_val in patterns {
        // Evaluate each pattern expression
        let sql_val = eval_expr(env, pattern_val)?;
        let sql = sql_val.as_str()
            .ok_or_else(|| AstError::TypeError("Pattern must evaluate to SQL string".to_string()))?;

        // Extract WHERE clause
        let re = regex::RegexBuilder::new(r"where\s+(.+?)\s+offset")
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .map_err(|e| AstError::EvalError(e.to_string()))?;

        let cond = if let Some(cap) = re.captures(sql) {
            format!("({})", cap.get(1).unwrap().as_str().trim())
        } else {
            sql.to_string()
        };
        conditions.push(cond);
    }

    let where_clause = conditions.join(" and \n    ");
    let sql = format!(
        "select {} \nfrom statement \nwhere \n    {} \noffset 0\nlimit 100 \n",
        QUERY_FIELDS, where_clause
    );

    Ok(Value::String(sql))
}

pub fn sql_functors() -> std::collections::HashMap<&'static str, Functor> {
    let mut m = std::collections::HashMap::new();
    m.insert("$pattern", pattern as Functor);
    m.insert("$expr", expr as Functor);
    m.insert("$query", query as Functor);
    m
}
