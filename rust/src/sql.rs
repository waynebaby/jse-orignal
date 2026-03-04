/// Query field list for SQL SELECT.
pub const QUERY_FIELDS: &str = "subject, predicate, object, meta";

/// Convert $pattern arguments to PostgreSQL jsonb containment triple.
pub fn pattern_to_triple(subject: &str, predicate: &str, object: &str) -> Vec<String> {
    if subject == "$*" && object == "$*" {
        return vec![predicate.to_string()];
    }
    vec![
        subject.to_string(),
        predicate.to_string(),
        object.to_string(),
    ]
}

/// Build SQL WHERE clause for a triple pattern.
pub fn triple_to_sql_condition(triple: &[String]) -> String {
    let json = serde_json::json!({ "triple": triple });
    let s = json.to_string();
    let escaped = s.replace('\'', "''");
    format!("meta @> '{escaped}'")
}
