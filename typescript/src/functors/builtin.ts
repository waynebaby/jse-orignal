/**
 * Basic builtin functors for JSE.
 *
 * Following docs/regular.md basic operators section:
 * - $quote: Quote unevaluated expression
 * - $eq: Equality comparison
 * - $cond: Multi-branch conditional
 * - $head/$tail: List car/cdr operations
 * - $atom?: Check if value is JSON atom type
 * - $cons: Construct list (Clojure-style)
 */

import type { JseValue } from "../types.js";
import type { Env } from "../env.js";

// Type alias for functors
export type Functor = (env: Env, ...args: JseValue[]) => JseValue;

/**
 * Return argument without evaluation.
 */
export function _quote(env: Env, ...args: JseValue[]): JseValue {
  return args[0] ?? null;
}

/**
 * Compare two values for equality.
 */
export function _eq(env: Env, ...args: JseValue[]): JseValue {
  if (args.length !== 2) {
    throw new Error("$eq requires exactly 2 arguments");
  }
  return args[0] === args[1];
}

/**
 * Multi-branch conditional evaluation.
 * Forms:
 * - [$cond, test1, result1, test2, result2, ...]
 * - [$cond, test1, result1, test2, result2, ..., default]
 */
export function _cond(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return null;
  }

  let pairs: [JseValue, JseValue][];
  let defaultCase: JseValue | null = null;

  if (args.length % 2 === 0) {
    // Pairs only: [test1, result1, test2, result2, ...]
    pairs = [];
    for (let i = 0; i < args.length; i += 2) {
      pairs.push([args[i], args[i + 1]]);
    }
  } else {
    // With default: [test1, result1, ..., default]
    pairs = [];
    for (let i = 0; i < args.length - 1; i += 2) {
      pairs.push([args[i], args[i + 1]]);
    }
    defaultCase = args[args.length - 1];
  }

  for (const [test, result] of pairs) {
    if (env.eval(test)) {
      return env.eval(result);
    }
  }

  return defaultCase !== null ? env.eval(defaultCase) : null;
}

/**
 * Get first element of list (car).
 */
export function _head(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    throw new Error("$head requires a list argument");
  }

  const lst = env.eval(args[0]);
  if (!Array.isArray(lst)) {
    throw new Error("$head requires a list argument");
  }
  if (lst.length === 0) {
    throw new Error("$head requires non-empty list");
  }

  return lst[0];
}

/**
 * Get rest of list (cdr).
 */
export function _tail(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    throw new Error("$tail requires a list argument");
  }

  const lst = env.eval(args[0]);
  if (!Array.isArray(lst)) {
    throw new Error("$tail requires a list argument");
  }

  return lst.length > 1 ? lst.slice(1) : [];
}

/**
 * Check if value is a JSON atom type.
 * Atoms are: number, string, boolean, null
 */
export function _atomp(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    throw new Error("$atom? requires an argument");
  }

  const value = env.eval(args[0]);
  return (
    value === null ||
    typeof value === "number" ||
    typeof value === "boolean" ||
    typeof value === "string"
  );
}

/**
 * Conjoin element and list (Clojure-style).
 * First argument: any element
 * Second argument: must be a list
 * Returns: new list [element, ...list]
 */
export function _cons(env: Env, ...args: JseValue[]): JseValue {
  if (args.length !== 2) {
    throw new Error("$cons requires exactly 2 arguments");
  }

  const elem = env.eval(args[0]);
  const lst = env.eval(args[1]);

  if (!Array.isArray(lst)) {
    throw new Error("$cons second argument must be a list");
  }

  return [elem, ...lst];
}

// Dict of all builtin functors for registration
export const BUILTIN_FUNCTORS: Record<string, Functor> = {
  $quote: _quote,
  $eq: _eq,
  $cond: _cond,
  $head: _head,
  $tail: _tail,
  "$atom?": _atomp,
  $cons: _cons,
};
