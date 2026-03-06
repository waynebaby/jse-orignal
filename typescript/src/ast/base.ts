/**
 * Base AST node class for JSE.
 *
 * Following the design in docs/regular.md:
 * - AST nodes contain env (passed during construction)
 * - AST nodes have getEnv() method
 * - AST nodes have apply() method (execution)
 * - The two-env pattern (construct-time vs call-time) enables static scoping
 */

import type { JseValue } from "../types.js";
import type { Env } from "../env.js";

/**
 * Abstract base class for all JSE AST nodes.
 *
 * Each AST node stores its construct-time environment (_env), which is
 * used for implementing closures with static scoping. When the node is
 * executed via apply(), a call-time environment is passed in.
 *
 * This two-env pattern is key to achieving lexical scoping:
 * - Construct-time env: Captured when node is created (for closures)
 * - Call-time env: Passed during execution (for parameters, etc.)
 */
export abstract class AstNode {
  protected _env: Env | null;

  constructor(env: Env | null = null) {
    this._env = env;
  }

  /**
   * Get the construct-time environment of this node.
   * For closures, this returns the captured environment.
   */
  getEnv(): Env | null {
    return this._env;
  }

  /**
   * Execute this AST node with the given call-time environment.
   *
   * @param env - The environment during execution (call-time scope).
   *              This may be different from getEnv() for closures.
   * @param args - Optional arguments for nodes like LambdaNode
   * @returns The result of executing this node
   */
  abstract apply(env: Env, ...args: JseValue[]): JseValue;
}
