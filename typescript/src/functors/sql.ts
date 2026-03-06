/**
 * SQL extension functors for JSE.
 *
 * Migrated from the original engine implementation:
 * - $pattern: Generate SQL for triple pattern matching
 * - $query: Generate SQL for multi-pattern queries
 */

import type { JseValue } from "../types.js";
import type { Env } from "../env.js";
import { patternToTriple, tripleToSqlCondition, QUERY_FIELDS } from "../sql.js";

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

  return `select \n    subject, predicate, object, meta \nfrom statement as s \nwhere ${cond} \noffset 0\nlimit 100 \n`;
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
 * Generate SQL for multi-pattern query.
 * Form: [$query, op, patterns]
 * where patterns is a list of SQL strings from $pattern
 */
export function _query(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 2) {
    throw new Error("$query expects [op, patterns array]");
  }

  // First arg is operator (currently ignored, assumes "and")
  const _op = env.eval(args[0]);
  const patterns = env.eval(args[1]);

  if (!Array.isArray(patterns)) {
    throw new Error("$query second argument must be a list");
  }

  const conditions: string[] = [];
  for (const sql of patterns) {
    if (typeof sql !== "string") {
      throw new Error("Pattern must evaluate to SQL string");
    }
    const match = sql.match(/where\s+(.+?)\s+offset/is);
    conditions.push(match ? `(${match[1].trim()})` : sql);
  }

  const whereClause = conditions.join(" and \n    ");
  return `select ${QUERY_FIELDS} \nfrom statement \nwhere \n    ${whereClause} \noffset 0\nlimit 100 \n`;
}

// Dict of all SQL functors for registration
export const SQL_FUNCTORS: Record<string, Functor> = {
  $pattern: _pattern,
  $expr: _expr,
  $query: _query,
};
