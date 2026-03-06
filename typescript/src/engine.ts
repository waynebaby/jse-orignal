/**
 * JSE execution engine with AST-based architecture.
 *
 * Following the design in docs/regular.md:
 * - Engine uses Parser to convert JSON to AST
 * - AST nodes are evaluated via env.eval() -> ast.apply()
 * - Supports static scoping through closure environments
 */

import type { JseValue } from "./types.js";
import type { Env } from "./env.js";
import { Parser } from "./ast/parser.js";

/**
 * JSE expression interpreter with AST-based execution.
 *
 * The engine parses JSON expressions into AST nodes, then evaluates
 * them using the environment's eval() method (which delegates to
 * each AST node's apply() method).
 *
 * This architecture enables:
 * - Static scoping (closures capture construct-time environment)
 * - Proper separation of parsing and evaluation
 * - Modular functor loading via env.load()
 */
export class Engine {
  private _parser: Parser;

  constructor(public env: Env) {
    this._parser = new Parser(env);
  }

  /**
   * Execute a JSE expression.
   *
   * Parses the expression into an AST, then evaluates it using
   * the environment's eval() method.
   *
   * @param expr - JSON expression to execute
   * @returns Result of evaluation
   */
  execute(expr: JseValue): JseValue {
    // Parse into AST
    const ast = this._parser.parse(expr);

    // Evaluate using environment (delegates to ast.apply())
    return this.env.eval(ast);
  }
}
