/**
 * Utility functors for JSE.
 *
 * Following docs/regular.md utils section:
 * - $not: Logical negation
 * - $list?/$map?/$null?: Type predicates
 * - $get/$set/$del: Container operations
 * - $conj: Append to list (Clojure-style)
 * - $and/$or: Logical operators
 * - $eq: Enhanced equality comparison
 */

import type { JseValue } from "../types.js";
import type { Env } from "../env.js";

// Type alias for functors
export type Functor = (env: Env, ...args: JseValue[]) => JseValue;

/**
 * Logical negation.
 */
export function _not(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return true;
  }
  const value = env.eval(args[0]);
  return !Boolean(value);
}

/**
 * Check if value is a list.
 */
export function _listp(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return false;
  }
  const value = env.eval(args[0]);
  return Array.isArray(value);
}

/**
 * Check if value is a map (dict).
 */
export function _mapp(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return false;
  }
  const value = env.eval(args[0]);
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/**
 * Check if value is null.
 * Distinguishes null from false (unlike classic Lisp).
 */
export function _nullp(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    return true;
  }
  const value = env.eval(args[0]);
  return value === null;
}

/**
 * Get value from map or list by key/index.
 */
export function _get(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 2) {
    throw new Error("$get requires (collection, key) arguments");
  }

  const collection = env.eval(args[0]);
  const key = env.eval(args[1]);

  if (typeof collection === "object" && collection !== null && !Array.isArray(collection)) {
    return (collection as Record<string, JseValue>)[String(key)] ?? null;
  }
  if (Array.isArray(collection)) {
    if (typeof key !== "number") {
      throw new Error("$get on list requires integer index");
    }
    if (key < 0 || key >= collection.length) {
      throw new Error(`Index ${key} out of range for list of length ${collection.length}`);
    }
    return collection[key];
  }

  throw new Error("$get first argument must be map or list");
}

/**
 * Set value in map or list (mutates).
 */
export function _set(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 3) {
    throw new Error("$set requires (collection, key, value) arguments");
  }

  const collection = env.eval(args[0]);
  const key = env.eval(args[1]);
  const value = env.eval(args[2]);

  if (typeof collection === "object" && collection !== null && !Array.isArray(collection)) {
    (collection as Record<string, JseValue>)[String(key)] = value;
    return collection;
  }
  if (Array.isArray(collection)) {
    if (typeof key !== "number") {
      throw new Error("$set on list requires integer index");
    }
    if (key < 0 || key >= collection.length) {
      throw new Error(`Index ${key} out of range for list of length ${collection.length}`);
    }
    collection[key] = value;
    return collection;
  }

  throw new Error("$set first argument must be map or list");
}

/**
 * Delete key from map or index from list (mutates).
 */
export function _del(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 2) {
    throw new Error("$del requires (collection, key) arguments");
  }

  const collection = env.eval(args[0]);
  const key = env.eval(args[1]);

  if (typeof collection === "object" && collection !== null && !Array.isArray(collection)) {
    delete (collection as Record<string, JseValue>)[String(key)];
    return collection;
  }
  if (Array.isArray(collection)) {
    if (typeof key !== "number") {
      throw new Error("$del on list requires integer index");
    }
    if (key < 0 || key >= collection.length) {
      throw new Error(`Index ${key} out of range for list of length ${collection.length}`);
    }
    collection.splice(key, 1);
    return collection;
  }

  throw new Error("$del first argument must be map or list");
}

/**
 * Conjoin element to end of list (Clojure-style).
 * Note: This appends (unlike $cons which prepends).
 */
export function _conj(env: Env, ...args: JseValue[]): JseValue {
  if (args.length !== 2) {
    throw new Error("$conj requires exactly 2 arguments");
  }

  const elem = env.eval(args[0]);
  const lst = env.eval(args[1]);

  if (!Array.isArray(lst)) {
    throw new Error("$conj second argument must be a list");
  }

  return [...lst, elem];
}

/**
 * Logical AND.
 */
export function _and(_env: Env, ...args: JseValue[]): JseValue {
  return args.every((v) => Boolean(v));
}

/**
 * Logical OR.
 */
export function _or(_env: Env, ...args: JseValue[]): JseValue {
  return args.some((v) => Boolean(v));
}

/**
 * Enhanced equality comparison.
 * - 1 arg: returns true
 * - 2 args: returns whether two args are equal
 * - More args: all-equal comparison
 */
export function _eqEnhanced(_env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 1) {
    return true;
  }
  if (args.length === 2) {
    return args[0] === args[1];
  }
  for (let i = 0; i < args.length - 1; i++) {
    if (args[i] !== args[i + 1]) {
      return false;
    }
  }
  return true;
}

// Dict of all utility functors for registration
export const UTILS_FUNCTORS: Record<string, Functor> = {
  $not: _not,
  "$list?": _listp,
  "$map?": _mapp,
  "$null?": _nullp,
  $get: _get,
  $set: _set,
  $del: _del,
  $conj: _conj,
  $and: _and,
  $or: _or,
  $eq: _eqEnhanced,
};
