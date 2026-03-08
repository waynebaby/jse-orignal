/**
 * SQL extension functors for JSE.
 *
 * Migrated from the original engine implementation:
 * - $pattern: Generate SQL for triple pattern matching
 * - $query: Generate SQL for multi-pattern queries
 */

import type { JseValue, AstNodeLike } from "../types.js";
import { Env } from "../env.js";
import { Parser } from "../ast/parser.js";

/**
 * Query field list for SQL SELECT.
 */
export const QUERY_FIELDS = "subject, predicate, object, meta";

/**
 * Convert $pattern arguments to PostgreSQL jsonb containment triple.
 * - ["$pattern", "$*", "author of", "$*"] -> triple: ["author of"] (filters $*)
 * - ["$pattern", "Liu Xin", "author of", "$*"] -> triple: ["Liu Xin", "author of"]
 */
export function patternToTriple(
  subject: string,
  predicate: string,
  object: string
): unknown[] {
  const pattern: unknown[] = [];
  // Filter out $* wildcard
  if (subject !== "$*" && subject !== "") {
    pattern.push(subject);
  }
  if (predicate !== "$*" && predicate !== "") {
    pattern.push(predicate);
  }
  if (object !== "$*" && object !== "") {
    pattern.push(object);
  }
  return pattern;
}

/**
 * Convert $pattern arguments for $query's local environment.
 * - ["pattern", "$*", "author of", "$*"] -> triple: ["*", "author of", "*"] (expands $*)
 * - ["pattern", "Liu Xin", "author of", "$*"] -> triple: ["Liu Xin", "author of", "*"]
 */
function patternToTripleForQuery(
  subject: string,
  predicate: string,
  object: string
): unknown[] {
  const pattern: unknown[] = [];
  // Expand $* wildcard to "*" string
  if (subject === "$*") {
    pattern.push("*");
  } else if (subject !== "") {
    pattern.push(subject);
  }
  if (predicate === "$*") {
    pattern.push("*");
  } else if (predicate !== "") {
    pattern.push(predicate);
  }
  if (object === "$*") {
    pattern.push("*");
  } else if (object !== "") {
    pattern.push(object);
  }
  return pattern;
}

/**
 * Build SQL WHERE clause for a triple pattern.
 */
export function tripleToSqlCondition(triple: unknown[]): string {
  const json = JSON.stringify({ triple });
  // Add spaces after colons for readability (matching Python output)
  const spaced = json.replace(/:/g, ": ");
  const escaped = spaced.replace(/'/g, "''");
  return `meta @> '${escaped}'`;
}

// Type alias for functors
export type Functor = (env: Env, ...args: JseValue[]) => JseValue;

/**
 * Generate SQL for triple pattern matching.
 * Form: [$pattern, subject, predicate, object]
 */
export function _pattern(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 3) {
    throw new Error("$pattern requires (subject, predicate, object)");
  }

  const subj = env.eval(args[0]);
  const pred = env.eval(args[1]);
  const obj = env.eval(args[2]);

  if (
    typeof subj !== "string" ||
    typeof pred !== "string" ||
    typeof obj !== "string"
  ) {
    throw new Error("$pattern requires string arguments");
  }

  const triple = patternToTriple(subj, pred, obj);
  const cond = tripleToSqlCondition(triple);

  return cond
}

/**
 * Expression evaluation pass-through.
 * Form: [$expr, expression] or {$expr: expression}
 * Evaluates the expression and returns the result.
 */
export function _expr(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return null;
  }
  return env.eval(args[0]);
}

/**
 * SQL-specific AND: joins conditions with " and ".
 * This is LOCAL-ONLY for $query, different from logical _and in utils.ts.
 */
function _and(env: Env, ...args: JseValue[]): JseValue {
  const tokens = args.map((e) => env.eval(e));
  return tokens.join(" and ");
}

/**
 * Wildcard helper for local scope.
 */
function _wildcard(_env: Env, ..._args: JseValue[]): JseValue {
  return "*";
}

/**
 * Local $pattern for $query's environment.
 * Expands $* to "*" in most cases, but filters when all args are $*.
 */
function _patternForQuery(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 3) {
    throw new Error("$pattern requires (subject, predicate, object)");
  }

  const subj = env.eval(args[0]);
  const pred = env.eval(args[1]);
  const obj = env.eval(args[2]);

  if (
    typeof subj !== "string" ||
    typeof pred !== "string" ||
    typeof obj !== "string"
  ) {
    throw new Error("$pattern requires string arguments");
  }

  // Special case: if all arguments are $*, filter them out
  if (subj === "$*" && pred === "$*" && obj === "$*") {
    const triple = patternToTriple(subj, pred, obj);  // Filters $*
    const cond = tripleToSqlCondition(triple);
    return cond;
  }

  // Otherwise, expand $* to "*"
  const triple = patternToTripleForQuery(subj, pred, obj);
  const cond = tripleToSqlCondition(triple);
  return cond;
}

/**
 * Generate SQL for multi-pattern query.
 * Form: [$query, condition]
 * where condition is an AST expression with local operators ($pattern, $and, $*)
 */
export function _query(env: Env, ...args: JseValue[]): JseValue {
  // Create local environment inheriting from parent
  const local = new Env(env);
  local.load({
    "$pattern": _patternForQuery,  // Use local version that expands $*
    "$and": _and, // SQL-specific AND
    "$*": _wildcard,
  });

  const parser = new Parser(local);
  // Parse all args as a single expression (like Python's tuple unpacking)
  const condition = parser.parse(args) as AstNodeLike;
  const where = condition.apply(local); // Evaluate in local env

  return `select ${QUERY_FIELDS} \nfrom statement \nwhere \n    ${where} \noffset 0\nlimit 100 \n`;
}

// Dict of all SQL functors for registration
export const SQL_FUNCTORS: Record<string, Functor> = {
  $query: _query,
};
