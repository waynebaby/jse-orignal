/**
 * LISP-enhanced functors for JSE.
 *
 * Following docs/regular.md lisp section:
 * - $apply: Apply functor to argument list
 * - $eval: Evaluate expression
 * - $lambda: Create lambda function with closure
 * - $def: Define symbol in current environment
 * - $defn: Define named function (def + lambda syntax sugar)
 */

import type { JseValue, Functor } from "../types.js";
import type { Env } from "../env.js";
import { LambdaNode } from "../ast/nodes.js";

/**
 * Apply functor to argument list.
 */
export function _apply(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 2) {
    throw new Error("$apply requires (functor, arglist) arguments");
  }

  const functor = env.eval(args[0]);
  const arglist = env.eval(args[1]);

  if (!Array.isArray(arglist)) {
    throw new Error("$apply second argument must be a list");
  }

  if (typeof functor !== "function") {
    throw new Error("$apply first argument must be callable");
  }

  return functor(env, ...arglist);
}

/**
 * Evaluate an expression.
 */
export function _evalExpr(env: Env, ...args: JseValue[]): JseValue {
  if (args.length === 0) {
    throw new Error("$eval requires an expression argument");
  }

  return env.eval(args[0]);
}

/**
 * Extract parameter names from potentially unevaluated expression.
 * Could be ArrayNode (list), SymbolNode (symbol), or already evaluated list.
 */
function extractParamNames(paramsExpr: JseValue): string[] {
  // Check if it has _elements property (ArrayNode)
  if (typeof paramsExpr === "object" && paramsExpr !== null && "_elements" in paramsExpr) {
    const elements = (paramsExpr as { _elements: JseValue[] })._elements;
    const names: string[] = [];
    for (const p of elements) {
      // Check if it's a SymbolNode
      if (typeof p === "object" && p !== null && "_name" in p) {
        names.push((p as { _name: string })._name);
      } else if (typeof p === "string") {
        names.push(p);
      } else {
        throw new Error(`$lambda parameters must be symbols, got: ${typeof p}`);
      }
    }
    return names;
  }

  // Check if it's a single SymbolNode
  if (typeof paramsExpr === "object" && paramsExpr !== null && "_name" in paramsExpr) {
    return [(paramsExpr as { _name: string })._name];
  }

  // Already evaluated list
  if (Array.isArray(paramsExpr)) {
    return paramsExpr as string[];
  }

  throw new Error("$lambda first argument must be a parameter list");
}

/**
 * Create a lambda function with closure.
 * Captures current environment for static scoping.
 */
export function _lambda(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 2) {
    throw new Error("$lambda requires (params, body) arguments");
  }

  const paramsExpr = args[0];
  const body = args.length > 1 ? args[1] : null;

  // Extract parameter names from potentially unevaluated expression
  const paramNames = extractParamNames(paramsExpr);

  // Validate all parameter names start with $
  for (const name of paramNames) {
    if (!name.startsWith("$")) {
      throw new Error(`$lambda parameters must be symbols starting with $, got: ${name}`);
    }
  }

  // Create lambda with current environment as closure (static scoping!)
  return new LambdaNode(paramNames, body, env);
}

/**
 * Extract name from potentially unevaluated expression.
 * Could be SymbolNode or string.
 */
function extractName(nameExpr: JseValue): string {
  // Check if it's a SymbolNode
  if (typeof nameExpr === "object" && nameExpr !== null && "_name" in nameExpr) {
    return (nameExpr as { _name: string })._name;
  }

  // String
  if (typeof nameExpr === "string") {
    if (!nameExpr.startsWith("$")) {
      throw new Error("First argument must be a symbol starting with $");
    }
    return nameExpr;
  }

  throw new Error("First argument must be a symbol or string");
}

/**
 * Define a symbol in current environment.
 */
export function _def(env: Env, ...args: JseValue[]): JseValue {
  if (args.length !== 2) {
    throw new Error("$def requires (name, value) arguments");
  }

  const nameExpr = args[0];
  const valueExpr = args[1];

  // Extract name
  const name = extractName(nameExpr);

  // Evaluate the value expression
  const value = env.eval(valueExpr);

  env.register(name, value);
  return value;
}

/**
 * Define a named function.
 * Syntactic sugar for: [$def, name, [$lambda, params, body]]
 */
export function _defn(env: Env, ...args: JseValue[]): JseValue {
  if (args.length < 3) {
    throw new Error("$defn requires (name, params, body) arguments");
  }

  const nameExpr = args[0];
  const paramsExpr = args[1];
  const body = args.length > 2 ? args[2] : null;

  // Extract name
  const name = extractName(nameExpr);

  // Create lambda using _lambda functor
  const lambdaFn = _lambda(env, paramsExpr, body) as LambdaNode;

  // Register it
  env.register(name, lambdaFn);
  return lambdaFn;
}

// Dict of all LISP functors for registration
export const LISP_FUNCTORS: Record<string, Functor> = {
  $apply: _apply,
  $eval: _evalExpr,
  $lambda: _lambda,
  $def: _def,
  $defn: _defn,
};
