using System.Collections.Concurrent;
using System.Linq.Expressions;
using Jse.Ast;
using Jse.Runtime;
 

namespace Jse.Execution;

public sealed class ExpressionCompiler
{
    private readonly ConcurrentDictionary<string, Delegate> _cache = new(StringComparer.Ordinal);

    public Func<object?> Compile(JseNode ast, OperatorSettings settings)
    {
        return Compile<object?>(ast, settings);
    }

    public Func<T> Compile<T>(JseNode ast, OperatorSettings settings)
    {
        var key = $"{typeof(T).FullName}:{BuildCacheKey(ast)}";
        return (Func<T>)_cache.GetOrAdd(key, _ => BuildDelegate<T>(ast, settings));
    }

    private static Func<T> BuildDelegate<T>(JseNode ast, OperatorSettings settings)
    {
        var body = BuildExpression(ast, settings);
        var lambda = Expression.Lambda<Func<T>>(ConvertIfNeeded(body, typeof(T)));
        return lambda.Compile();
    }

    private static Expression ConvertIfNeeded(Expression expression, Type targetType)
    {
        if (expression.Type == targetType)
        {
            return expression;
        }

        if (targetType.IsAssignableFrom(expression.Type) && !expression.Type.IsValueType)
        {
            return expression;
        }

        return Expression.Convert(expression, targetType);
    }

    private static Expression BuildExpression(JseNode node, OperatorSettings settings)
    {
        return node switch
        {
            JseLiteral literal => BuildLiteralExpression(literal.Value),
            JseSymbol symbol => Expression.Constant($"${symbol.Name}"),
            JseCall call => BuildCall(call, settings),
            _ => throw new InvalidOperationException($"Unsupported AST node: {node.GetType().Name}")
        };
    }

    private static Expression BuildLiteralExpression(object? value)
    {
        if (value is null)
        {
            return Expression.Constant(null, typeof(object));
        }

        return Expression.Constant(value, value.GetType());
    }

    private static Expression BuildCall(JseCall call, OperatorSettings settings)
    {
        if (string.Equals(call.Operator, "quote", StringComparison.Ordinal))
        {
            if (call.Args.Count != 1)
            {
                throw new InvalidOperationException("$quote requires exactly 1 argument.");
            }

            var quoted = QuoteNode(call.Args[0]);
            return BuildLiteralExpression(quoted);
        }

        var args = call.Args
            .Select(arg => BuildExpression(arg, settings))
            .ToArray();

        var argTypes = args.Select(static x => x.Type).ToArray();
        var binding = settings.Operators.Resolve(call.Operator, argTypes);
        var convertedArgs = args
            .Select((arg, i) => ConvertIfNeeded(arg, binding.ParameterTypes[i]))
            .ToArray();

        var opExpr = Expression.Constant(binding.Implementation, binding.Implementation.GetType());
        return Expression.Invoke(opExpr, convertedArgs);
    }

    private static object? QuoteNode(JseNode node)
    {
        return node switch
        {
            JseLiteral literal => literal.Value,
            JseSymbol symbol => $"${symbol.Name}",
            JseCall call => new object?[]
            {
                $"${call.Operator}"
            }.Concat(call.Args.Select(QuoteNode)).ToList(),
            _ => throw new InvalidOperationException($"Unsupported quoted AST node: {node.GetType().Name}")
        };
    }

    private static string BuildCacheKey(JseNode ast) => SerializeNode(ast);

    private static string SerializeNode(JseNode node)
    {
        return node switch
        {
            JseLiteral literal => $"lit:{SerializeValue(literal.Value)}",
            JseSymbol symbol => $"sym:{symbol.Name}",
            JseCall call => $"call:{call.Operator}({string.Join(',', call.Args.Select(SerializeNode))})",
            _ => throw new InvalidOperationException($"Unsupported AST node: {node.GetType().Name}")
        };
    }

    private static string SerializeValue(object? value)
    {
        return value switch
        {
            null => "null",
            string s => $"\"{s}\"",
            bool b => b ? "true" : "false",
            IEnumerable<object?> list => $"[{string.Join(',', list.Select(SerializeValue))}]",
            IDictionary<string, object?> dict => $"{{{string.Join(',', dict.OrderBy(k => k.Key).Select(kv => $"{kv.Key}:{SerializeValue(kv.Value)}"))}}}",
            _ => value.ToString() ?? string.Empty
        };
    }
}
