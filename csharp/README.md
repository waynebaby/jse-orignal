# JSE C# Runtime (`src/csharp`)

A .NET 8 implementation of JSE using:

- `System.Text.Json` for JSON parsing
- `System.Linq.Expressions` for expression-tree compilation

## Structure

- `Jse/Ast` immutable AST nodes
- `Jse/Parser` parser from `JsonElement` to AST
- `Jse/Runtime` engine/environment/operator registry
- `Jse/Execution` expression compiler with delegate cache
- `Jse.Tests` xUnit tests

## Example

```csharp
using Jse.Runtime;

var engine = new JseEngine();
var result = engine.Execute("[\"$add\",1,2]");
// result == 3
```

## Notes

- Execution is restricted to registered operators.
- No reflection/dynamic code generation is used.
- `$quote` returns unevaluated JSON-shaped data.
- Test catalog and stress scenarios are documented in `TESTING.md`.

## C# Specific Semantics

- JSON values are mapped by `JsonHelpers` as:
	- number -> `decimal`
	- array -> `List<object?>`
	- object -> `Dictionary<string, object?>`
- Parser exposes reusable JSON configuration via `JseParser.SerializerOptions` and `JseParser.NodeConverter`.
- Compiler supports `Compile<T>` for typed delegate generation; `JseEngine.Execute` remains `object?` at the API boundary.
- Operator registry uses typed bindings and overload resolution by argument types.
- Operator registration accepts both specific and general signatures (including `object`/`object?`), and runtime selects the most specific applicable overload.
