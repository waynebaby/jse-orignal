use serde_json::{Map, Value};

use crate::env::Env;
use crate::sql::{pattern_to_triple, triple_to_sql_condition, QUERY_FIELDS};

/// JSE execution error.
#[derive(Debug, thiserror::Error)]
pub enum JseError {
    #[error("{0}")]
    Message(String),
}

// Avoid pulling thiserror if we want zero extra deps; use a simple impl instead.
impl JseError {
    fn msg(s: impl Into<String>) -> Self {
        JseError::Message(s.into())
    }
}

fn is_symbol(s: &str) -> bool {
    s.starts_with('$') && !s.starts_with("$$")
}

fn unescape_symbol(s: &str) -> String {
    if s.starts_with("$$") {
        s[1..].to_string()
    } else {
        s.to_string()
    }
}

fn get_s_expr_key(obj: &Map<String, Value>) -> Option<String> {
    let mut found = None;
    for k in obj.keys() {
        if k.starts_with('$') && !k.starts_with("$$") {
            if found.is_some() {
                return None;
            }
            found = Some(k.clone());
        }
    }
    found
}

/// JSE execution engine.
pub struct Engine<E: Env> {
    env: E,
}

impl<E: Env> Engine<E> {
    pub fn new(env: E) -> Self {
        Self { env }
    }

    /// Execute a JSE expression.
    pub fn execute(&self, expr: &Value) -> Result<Value, JseError> {
        match expr {
            Value::Null | Value::Bool(_) | Value::Number(_) => Ok(expr.clone()),
            Value::String(s) => Ok(Value::String(unescape_symbol(s))),
            Value::Array(arr) => {
                if arr.is_empty() {
                    return Ok(expr.clone());
                }
                let first = arr.first().unwrap();
                if let Some(sym) = first.as_str().filter(|s| is_symbol(s)) {
                    let tail: Vec<_> = arr[1..].to_vec();
                    return self.eval_s_expr(sym, &tail);
                }
                let out: Result<Vec<_>, _> = arr.iter().map(|e| self.execute(e)).collect();
                Ok(Value::Array(out?))
            }
            Value::Object(obj) => {
                if let Some(sym) = get_s_expr_key(obj) {
                    let tail = obj.get(&sym).cloned().unwrap_or(Value::Null);
                    let args = if sym == "$expr" {
                        vec![tail]
                    } else if let Value::Array(a) = tail {
                        a
                    } else {
                        vec![tail]
                    };
                    return self.eval_s_expr(&sym, &args);
                }
                let mut result = Map::new();
                for (k, v) in obj {
                    let key = unescape_symbol(k);
                    result.insert(key, self.execute(v)?);
                }
                Ok(Value::Object(result))
            }
        }
    }

    fn eval_s_expr(&self, symbol: &str, args: &[Value]) -> Result<Value, JseError> {
        if symbol == "$quote" {
            return Ok(args.first().cloned().unwrap_or(Value::Null));
        }

        let evaluated: Result<Vec<_>, _> = args.iter().map(|a| self.execute(a)).collect();
        let evaluated = evaluated?;

        match symbol {
            "$and" => Ok(Value::Bool(evaluated.iter().all(|v| as_bool(v)))),
            "$or" => Ok(Value::Bool(evaluated.iter().any(|v| as_bool(v)))),
            "$not" => Ok(Value::Bool(!as_bool(evaluated.first().unwrap_or(&Value::Null)))),
            "$expr" => Ok(evaluated.first().cloned().unwrap_or(Value::Null)),
            "$pattern" => self.eval_pattern(&evaluated),
            "$query" => self.eval_query(&evaluated),
            _ => {
                if let Some(v) = self.env.resolve(symbol) {
                    Ok(v)
                } else {
                    Err(JseError::msg(format!("Unknown symbol: {symbol}")))
                }
            }
        }
    }

    fn eval_pattern(&self, evaluated: &[Value]) -> Result<Value, JseError> {
        let subj = evaluated
            .get(0)
            .and_then(Value::as_str)
            .ok_or_else(|| JseError::msg("$pattern requires (subject, predicate, object)"))?;
        let pred = evaluated
            .get(1)
            .and_then(Value::as_str)
            .ok_or_else(|| JseError::msg("$pattern requires (subject, predicate, object)"))?;
        let obj = evaluated
            .get(2)
            .and_then(Value::as_str)
            .ok_or_else(|| JseError::msg("$pattern requires (subject, predicate, object)"))?;
        let triple = pattern_to_triple(subj, pred, obj);
        let cond = triple_to_sql_condition(&triple);
        let sql = format!(
            "select \n    subject, predicate, object, meta \nfrom statement as s \nwhere {} \noffset 0\nlimit 100 \n",
            cond
        );
        Ok(Value::String(sql))
    }

    fn eval_query(&self, evaluated: &[Value]) -> Result<Value, JseError> {
        let patterns = evaluated
            .get(1)
            .and_then(Value::as_array)
            .ok_or_else(|| JseError::msg("$query expects [op, patterns array]"))?;
        let re = regex::RegexBuilder::new(r"where\s+(.+?)\s+offset")
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .map_err(|e| JseError::msg(e.to_string()))?;
        let mut conditions = Vec::new();
        for sql_val in patterns {
            let sql = sql_val
                .as_str()
                .ok_or_else(|| JseError::msg("Pattern must evaluate to SQL string"))?;
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
}

fn as_bool(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Null => false,
        _ => true,
    }
}