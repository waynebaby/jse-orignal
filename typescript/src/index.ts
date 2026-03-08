// Core exports
export { Engine } from "./engine.js";
export { Env, ExpressionEnv } from "./env.js";
export type { IEnv } from "./env.js";
export type { JseValue, Functor, AstNodeLike } from "./types.js";

// AST exports
export { AstNode } from "./ast/base.js";
export {
  SymbolNode,
  ArrayNode,
  ObjectNode,
  ObjectExpressionNode,
  QuoteNode,
  LambdaNode,
  LiteralNode,
} from "./ast/nodes.js";
export { Parser } from "./ast/parser.js";

// Functor exports
export { BUILTIN_FUNCTORS } from "./functors/builtin.js";
export { UTILS_FUNCTORS } from "./functors/utils.js";
export { LISP_FUNCTORS } from "./functors/lisp.js";
export { SQL_FUNCTORS } from "./functors/sql.js";

// SQL exports
export { patternToTriple, tripleToSqlCondition, QUERY_FIELDS } from "./functors/sql.js";
