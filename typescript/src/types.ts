/**
 * JSE (JSON Structural Expression) value types.
 * - JSON primitives and structures
 * - $symbol strings (escape: $$)
 * - $quote for pass-through
 */

// Forward declaration to avoid circular import
export interface AstNodeLike {
  apply(env: any, ...args: any[]): any;
  getEnv(): any;
}

// Functor type
export type Functor = (env: any, ...args: any[]) => any;

export type JseValue =
  | number
  | string
  | boolean
  | null
  | JseValue[]
  | { [key: string]: JseValue }
  | AstNodeLike
  | Functor;
