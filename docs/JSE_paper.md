# JSE v2.0 — A JSON Structural Expression Language with AST and Static Scoping

Large Language Models are very good at producing JSON.

But JSON itself has a limitation: **it is purely data, not computation**.

This makes it difficult to express logic, transformations, or reasoning structures in a format that is:

* machine interpretable
* safe to evaluate
* friendly for LLM output

To address this problem, I created **JSE (JSON Structural Expression)** — a minimal expression system embedded directly inside JSON.

GitHub:
👉 [https://github.com/MarchLiu/jse](https://github.com/MarchLiu/jse)

Recently I released **JSE v2.0**, which introduces an AST-based architecture and proper lexical scoping while maintaining full backward compatibility with v1.0.

This post introduces the motivation, design, and what's new in v2.0.

---

# What is JSE?

**JSE (JSON Structural Expression)** is a structured expression language where programs are written entirely in **JSON values**.

Instead of writing:

```
add(1, 2)
```

You write:

```json
["$add", 1, 2]
```

This makes expressions:

* JSON-native
* deterministic to parse
* easy for LLMs to generate
* safe to sandbox

The core design idea is simple:

> Treat JSON as an abstract syntax tree.

---

# Why JSE?

Many AI pipelines today rely on structured outputs such as:

* JSON tool calls
* JSON transformations
* structured reasoning chains

However, JSON lacks **computation semantics**.

Developers often resort to:

* embedding Python
* embedding JavaScript
* inventing DSLs

These approaches introduce problems:

| Problem            | Explanation                             |
| ------------------ | --------------------------------------- |
| Security           | Evaluating arbitrary code is dangerous  |
| Parsing complexity | Text-based DSLs require complex parsers |
| LLM reliability    | Free-form syntax is error-prone         |
| Tooling mismatch   | JSON pipelines expect structured data   |

JSE solves this by defining **a minimal expression language that *is itself JSON***.

---

# Example

A simple equality test:

```json
["$eq", 1, 1]
```

Result:

```
true
```

Conditional logic:

```json
["$cond", true, "yes", "no"]
```

Result:

```
"yes"
```

Lists:

```json
["$cons", 1, [2,3]]
```

Result:

```
[1,2,3]
```

---

# Core Design Principles

JSE was designed with several constraints in mind:

### 1. JSON-first

Every expression must be valid JSON.

This allows:

* native parsing
* safe transport
* easy LLM generation

---

### 2. Structural evaluation

JSE evaluates **structure**, not text.

Example:

```
JSON → AST → evaluation
```

There is no string parsing during execution.

---

### 3. Minimal operators

The language intentionally starts small.

Core operators include:

| Operator | Description        |
| -------- | ------------------ |
| `$quote` | Prevent evaluation |
| `$eq`    | Equality           |
| `$cond`  | Conditional        |
| `$head`  | First element      |
| `$tail`  | Rest of list       |
| `$cons`  | List construction  |

Additional modules provide utilities and Lisp-style features.

---

# What's New in JSE v2.0

Version 2.0 introduces a more formal execution model.

The biggest changes are:

### 1️⃣ AST-based architecture

Expressions are now parsed into an **Abstract Syntax Tree** before evaluation.

Execution pipeline:

```
JSON → Parser → AST → Environment → Result
```

Benefits:

* deterministic evaluation
* easier extension
* better tooling support

---

### 2️⃣ Static scoping

v2.0 introduces **lexical scoping**, enabling closures.

Functions now capture the environment where they were defined.

Example:

```json
{
  "$lambda": {
    "params": ["x"],
    "body": ["$add", "$x", 1]
  }
}
```

This returns a function that remembers its definition context.

---

### 3️⃣ First-class functions

JSE now supports:

```
$lambda
$def
$defn
```

Example:

```json
["$defn", "add", ["x","y"], ["$add", "$x", "$y"]]
```

This defines a function `add`.

---

### 4️⃣ Scope chains

Environments now support parent relationships:

```
Environment
 ├─ bindings
 ├─ functors
 └─ parent
```

Symbol resolution follows the chain.

This enables:

* closures
* modules
* nested scopes

---

### 5️⃣ Object expression syntax

JSE also supports an **object-form operator syntax**:

```json
{
  "$pattern": ["$*", "author of", "$*"],
  "confidence": 0.95
}
```

Rules:

* exactly one `$` key
* remaining keys are metadata
* metadata is preserved

This is useful for **AI reasoning outputs**.

---

# Module System

Operators are organized into modules:

### builtin

Core language primitives.

```
$quote
$eq
$cond
$head
$tail
$atom?
$cons
```

---

### utils

General helpers.

```
$not
$and
$or
$list?
$map?
$get
$set
$del
$conj
```

---

### lisp

Functional programming extensions.

```
$def
$defn
$lambda
```

---

# Example: Recursive Function

Factorial in JSE:

```json
{
  "$defn": "factorial",
  ["n"],
  ["$cond",
    ["$eq", "$n", 0],
    1,
    ["$mul", "$n", ["$factorial", ["$add", "$n", -1]]]
  ]
}
```

---

# AI-Friendly Design

JSE was designed with **LLM output reliability** in mind.

AI systems generating JSE must follow simple rules:

* always produce valid JSON
* operators begin with `$`
* use `$quote` to prevent evaluation
* never include multiple operator keys in one object

This structural constraint dramatically improves generation reliability compared to free-form DSLs.

---

# Security Model

Implementations can whitelist operators.

Recommended minimal set:

```
$quote
$eq
$cond
```

A safe production set might include:

```
builtin + utils
```

Advanced features like `$def` or `$lambda` can be disabled if necessary.

---

# Implementations

Current implementations include:

* Rust
* Java

Java artifact:
`io.github.marchliu:jse4j` ([Maven Central][1])

The project is MIT licensed.

---

# Potential Use Cases

JSE works well in environments where **structured expressions are required**:

### AI pipelines

LLM outputs that contain executable logic.

### rule engines

Declarative rule evaluation.

### data transformations

JSON-to-JSON computation.

### query planning

Intermediate representation for query engines.

### tool orchestration

Declarative tool invocation trees.

---

# Future Directions

Planned ideas include:

* streaming evaluation
* additional modules (math, SQL-like operators)
* WASM runtime
* LLM-native tooling

---

# Repository

GitHub:

[https://github.com/MarchLiu/jse](https://github.com/MarchLiu/jse)

Feedback and contributions are welcome.

---

If you're interested in **structured computation for AI outputs**, I'd love to hear your thoughts.


