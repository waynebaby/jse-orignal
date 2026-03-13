# C# Testing Guide

This document describes all C# tests under `csharp/Jse.Tests`.

## Scope

- Language/runtime: C# (.NET 8)
- Test framework: xUnit
- Test focus:
  - JSE functional semantics
  - typed operator overload resolution
  - stress and stability scenarios
  - serializer round-trip correctness
  - known recursion-depth behavior

## JSON Type Policy In Tests

- Stress tests should use JSON-compatible runtime value types.
- Numeric stress tests use `decimal` to align with current C# JSE number mapping.
- Avoid non-JSON numeric types (for example `BigInteger`) in standard stress assertions.

## Run Tests

```bash
dotnet test csharp/Jse.CSharp.sln
```

### Run by file filter

```bash
dotnet test csharp/Jse.CSharp.sln --filter "FullyQualifiedName~Jse.Tests.JseEngineTests"
dotnet test csharp/Jse.CSharp.sln --filter "FullyQualifiedName~Jse.Tests.JseStressTests"
```

### Run stress tests only

```bash
dotnet test csharp/Jse.CSharp.sln --filter "FullyQualifiedName~Jse.Tests.JseStressTests"
```

### Run a single test

```bash
dotnet test csharp/Jse.CSharp.sln --filter "FullyQualifiedName~Execute_PiApproximation_GeneratedTree_RoundsToFourDecimals"
```

## Test Files

- `csharp/Jse.Tests/JseEngineTests.cs`
- `csharp/Jse.Tests/JseStressTests.cs`
- `csharp/Jse.Tests/JsonValueExtensions.cs`

## Functional Tests (`JseEngineTests`)

- `Execute_LiteralValues_ReturnsOriginal`
  - Verifies literal execution for number/string/bool.
- `Execute_SimpleExpression_Addition`
  - Verifies basic expression execution with `$add`.
- `Execute_NestedExpression_Addition`
  - Verifies nested call evaluation.
- `Execute_NamedForm_Addition`
  - Verifies object-form operator invocation and metadata tolerance.
- `Execute_Quote_PreventsEvaluation`
  - Verifies `$quote` returns unevaluated data shape.
- `Execute_SymbolEscape_ReturnsLiteralDollarString`
  - Verifies `$$` escape semantics.
- `Execute_WithJsonElementInput_Works`
  - Verifies `JsonElement` execution path.

## Stress Tests (`JseStressTests`)

All stress tests follow this policy:

1. Execute once with original generated tree and operator settings.
2. Serialize `JseNode` and `OperatorSettings`.
3. Deserialize both.
4. Execute again with deserialized artifacts.
5. Compare results (or compare expected failure mode).

- `Execute_FactorialDepth10_GeneratedTree_ReturnsExpectedDecimal`
  - Generates a multiply tree and validates exact `10!` as `decimal` (`3628800m`).
- `Execute_PiApproximation_GeneratedTree_RoundsToFourDecimals`
  - Generates Nilakantha-series tree and validates pi rounded to 4 decimals.
- `Resolve_AmbiguousGeneralOverloads_Throws`
  - Validates overload ambiguity detection (`IComparable` vs `IFormattable`).
- `Execute_DeepChainSum300_GeneratedTree_ReturnsExpected`
  - Validates non-trivial deep left-chain execution under safe depth.
- `Compile_RepeatedEquivalentAst_ReusesCachedDelegate`
  - Validates delegate cache reuse for equivalent AST + target type.
- `Execute_WideBalancedSum1024_GeneratedTree_ReturnsExpected`
  - Validates wide balanced-tree execution (many nodes, low depth).
- `Execute_DeepChainSum2000_GeneratedTree_KnownStackDepthLimitation` (Skipped)
  - Documents known stack-depth limitation for very deep left-leaning trees.

## Serialization APIs Under Test

- `Jse.Serialization.JseRuntimeSerializer.SerializeNode`
- `Jse.Serialization.JseRuntimeSerializer.DeserializeNode`
- `Jse.Serialization.JseRuntimeSerializer.SerializeOperatorSettings`
- `Jse.Serialization.JseRuntimeSerializer.DeserializeOperatorSettings`

### Stress Matrix

- Depth stress:
  - `Execute_DeepChainSum300_GeneratedTree_ReturnsExpected`
- Width stress:
  - `Execute_WideBalancedSum1024_GeneratedTree_ReturnsExpected`
- Numeric precision stress:
  - `Execute_PiApproximation_GeneratedTree_RoundsToFourDecimals`
- Compile/cache stress:
  - `Compile_RepeatedEquivalentAst_ReusesCachedDelegate`
- Error-path stress:
  - `Resolve_AmbiguousGeneralOverloads_Throws`

## Test Helper

- `JsonValueExtensions.JsonValue<T>(this T value) where T : struct`
  - Normalizes expected values to JSE JSON mapping semantics for assertions.
  - Number-like structs are normalized to `decimal`.

## Known Limitation

- Deep left-leaning generated expression trees can overflow stack during expression building/compilation.
- Preferred mitigation in tests and production generation:
  - Keep chain depth bounded, or
  - Build balanced trees.

## How To Add New Stress Cases

1. Keep generated trees deterministic (no unseeded randomness).
2. Use JSON-compatible value types in literals (`decimal`, `bool`, `string`, list/object shapes).
3. Include serialization round-trip replay in each stress case.
3. Keep runtime budget reasonable for CI/local runs.
4. Assert both value correctness and, when relevant, failure mode text.
5. If a case documents a known limit, mark as `Skip` with a clear reason.

### Checklist For New Tests

- Expression tree is generated in code.
- Expected value is deterministic.
- Uses typed operator registrations compatible with scenario.
- Prefer `RegisterExpression(...)` when the scenario validates operator settings serialization.
- Includes serializer round-trip replay and validation.
- No external dependencies introduced.
- Included in this document under the appropriate section.

## Suggested Next Cases

- Mixed overload precedence tests (`decimal` vs `object?` with equal arity).
- Cache behavior across different target delegate types (`Compile<object?>` vs `Compile<decimal>`).
- Quote-heavy payload regression tests with large nested arrays/objects.
